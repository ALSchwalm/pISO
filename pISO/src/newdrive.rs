use action;
use bitmap;
use controller;
use displaymanager::{DisplayManager, Position, Widget, Window, WindowId};
use error;
use font;
use input;
use lvm;
use render;
use usb;
use std::sync::{Arc, Mutex};

enum NewDriveState {
    Unselected,
    PickingSize(DriveSize),
}

pub struct NewDrive {
    pub window: WindowId,
    pub usb: Arc<Mutex<usb::UsbGadget>>,
    vg: lvm::VolumeGroup,
    state: NewDriveState,
}

impl NewDrive {
    pub fn new(
        disp: &mut DisplayManager,
        usb: Arc<Mutex<usb::UsbGadget>>,
        vg: lvm::VolumeGroup,
    ) -> error::Result<NewDrive> {
        let our_window = disp.add_child(Position::Normal)?;
        Ok(NewDrive {
            window: our_window,
            state: NewDriveState::Unselected,
            usb: usb,
            vg: vg,
        })
    }
}

impl render::Render for NewDrive {
    fn render(&self, window: &Window) -> error::Result<bitmap::Bitmap> {
        let mut base = bitmap::Bitmap::new(10, 1);
        base.blit(&font::render_text("New Drive"), (12, 0));
        if window.focus {
            base.blit(&bitmap::Bitmap::from_slice(font::ARROW), (0, 0));
        }
        Ok(base)
    }
}

impl input::Input for NewDrive {
    fn on_event(
        &mut self,
        event: &controller::Event,
    ) -> error::Result<(bool, Vec<action::Action>)> {
        match *event {
            controller::Event::Select => Ok((true, vec![action::Action::OpenSizeMenu])),
            _ => Ok((false, vec![])),
        }
    }

    fn do_action(
        &mut self,
        disp: &mut DisplayManager,
        action: &action::Action,
    ) -> error::Result<bool> {
        match *action {
            action::Action::OpenSizeMenu => {
                let menu = DriveSize::new(disp, self.usb.clone(), self.vg.clone())?;
                disp.shift_focus(&menu);
                self.state = NewDriveState::PickingSize(menu);
                Ok(true)
            }
            action::Action::CloseFormatMenu => {
                disp.shift_focus(self);
                self.state = NewDriveState::Unselected;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

impl Widget for NewDrive {
    fn mut_children(&mut self) -> Vec<&mut Widget> {
        match self.state {
            NewDriveState::PickingSize(ref mut widget) => vec![widget],
            NewDriveState::Unselected => vec![],
        }
    }

    fn children(&self) -> Vec<&Widget> {
        match self.state {
            NewDriveState::PickingSize(ref widget) => vec![widget],
            NewDriveState::Unselected => vec![],
        }
    }

    fn windowid(&self) -> WindowId {
        self.window
    }
}

enum DriveSizeState {
    Unselected,
    Selected(DriveFormat),
}

struct DriveSize {
    pub window: WindowId,
    pub current_percent: u32,
    pub usb: Arc<Mutex<usb::UsbGadget>>,
    vg: lvm::VolumeGroup,
    state: DriveSizeState,
}

impl DriveSize {
    fn new(
        disp: &mut DisplayManager,
        usb: Arc<Mutex<usb::UsbGadget>>,
        vg: lvm::VolumeGroup,
    ) -> error::Result<DriveSize> {
        Ok(DriveSize {
            window: disp.add_child(Position::Fixed(0, 0))?,
            current_percent: 50,
            usb: usb,
            vg: vg,
            state: DriveSizeState::Unselected,
        })
    }

    fn current_size(&self) -> u64 {
        let bytes = self.vg.report().expect("Failed to get vg report").vg_size as f32
            * (self.current_percent as f32 / 100.0);
        ((bytes as u64 + 512 - 1) / 512) * 512
    }
}

impl render::Render for DriveSize {
    fn render(&self, window: &Window) -> error::Result<bitmap::Bitmap> {
        let mut base = bitmap::Bitmap::new(0, 0);
        base.blit(&font::render_text("New drive capacity:"), (0, 0));

        let short_size = self.current_size() as f64 / (1024 * 1024 * 1024) as f64;
        base.blit(
            &font::render_text(format!("{}% ({:.2}GB)", self.current_percent, short_size)),
            (10, 30),
        );
        Ok(base)
    }
}

impl input::Input for DriveSize {
    fn on_event(
        &mut self,
        event: &controller::Event,
    ) -> error::Result<(bool, Vec<action::Action>)> {
        match *event {
            controller::Event::Select => Ok((true, vec![action::Action::OpenFormatMenu])),
            controller::Event::Up => Ok((true, vec![action::Action::IncDriveSize])),
            controller::Event::Down => Ok((true, vec![action::Action::DecDriveSize])),
        }
    }

    fn do_action(
        &mut self,
        disp: &mut DisplayManager,
        action: &action::Action,
    ) -> error::Result<bool> {
        match *action {
            action::Action::IncDriveSize => {
                self.current_percent += 5;
                Ok(true)
            }
            action::Action::DecDriveSize => {
                self.current_percent -= 5;
                Ok(true)
            }
            action::Action::OpenFormatMenu => {
                let menu = DriveFormat::new(disp, self.vg.clone(), self.current_size())?;
                disp.shift_focus(&menu);
                self.state = DriveSizeState::Selected(menu);
                Ok(true)
            }
            action::Action::CloseFormatMenu => {
                self.state = DriveSizeState::Unselected;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

impl Widget for DriveSize {
    fn mut_children(&mut self) -> Vec<&mut Widget> {
        match self.state {
            DriveSizeState::Selected(ref mut menu) => vec![menu],
            DriveSizeState::Unselected => vec![],
        }
    }

    fn children(&self) -> Vec<&Widget> {
        match self.state {
            DriveSizeState::Selected(ref menu) => vec![menu],
            DriveSizeState::Unselected => vec![],
        }
    }

    fn windowid(&self) -> WindowId {
        self.window
    }
}

struct DriveFormat {
    pub windowid: WindowId,
    vg: lvm::VolumeGroup,
    size: u64,
}

impl DriveFormat {
    fn new(
        disp: &mut DisplayManager,
        vg: lvm::VolumeGroup,
        size: u64,
    ) -> error::Result<DriveFormat> {
        Ok(DriveFormat {
            windowid: disp.add_child(Position::Fixed(0, 0))?,
            vg: vg,
            size: size,
        })
    }
}

impl render::Render for DriveFormat {
    fn render(&self, window: &Window) -> error::Result<bitmap::Bitmap> {
        let mut base = bitmap::Bitmap::new(0, 0);
        base.blit(&font::render_text("Select Format:"), (0, 0));
        Ok(base)
    }
}

impl input::Input for DriveFormat {
    fn on_event(
        &mut self,
        event: &controller::Event,
    ) -> error::Result<(bool, Vec<action::Action>)> {
        match *event {
            //TODO: create volume
            controller::Event::Select => {
                let count = self.vg.volumes()?.len();

                let volume = self.vg
                    .create_volume(&format!("Drive{}", count), self.size)?;

                Ok((
                    true,
                    vec![
                        action::Action::CloseFormatMenu,
                        action::Action::CreateDrive(volume),
                    ],
                ))
            }
            controller::Event::Up => Ok((true, vec![action::Action::NextFormat])),
            controller::Event::Down => Ok((true, vec![action::Action::PrevFormat])),
        }
    }

    fn do_action(
        &mut self,
        disp: &mut DisplayManager,
        action: &action::Action,
    ) -> error::Result<bool> {
        match *action {
            _ => Ok(false),
        }
    }
}

impl Widget for DriveFormat {
    fn mut_children(&mut self) -> Vec<&mut Widget> {
        vec![]
    }

    fn children(&self) -> Vec<&Widget> {
        vec![]
    }

    fn windowid(&self) -> WindowId {
        self.windowid
    }
}

use action;
use bitmap;
use controller;
use displaymanager::{DisplayManager, Position, Widget, Window, WindowId};
use error;
use font;
use input;
use render;
use std::sync::{Arc, Mutex};
use std::path;
use usb;

enum MountState {
    Mounted(usb::StorageID),
    Unmounted,
}

pub struct Iso {
    state: MountState,
    pub usb: Arc<Mutex<usb::UsbGadget>>,
    pub window: WindowId,
    pub path: path::PathBuf,
}

impl Iso {
    pub fn new(
        disp: &mut DisplayManager,
        usb: Arc<Mutex<usb::UsbGadget>>,
        path: path::PathBuf,
    ) -> error::Result<Iso> {
        let window = disp.add_child(Position::Normal)?;
        Ok(Iso {
            window: window,
            usb: usb,
            state: MountState::Unmounted,
            path: path,
        })
    }

    pub fn mount(&mut self) -> error::Result<()> {
        match self.state {
            MountState::Unmounted => {
                self.state = MountState::Mounted(self.usb.lock()?.export_file(&self.path, true)?);
                Ok(())
            }
            MountState::Mounted(_) => Err("Attempt to mount iso while already mounted".into()),
        }
    }

    pub fn unmount(&mut self) -> error::Result<()> {
        self.state = match self.state {
            MountState::Mounted(ref id) => {
                self.usb.lock()?.unexport_file(id)?;
                MountState::Unmounted
            }
            MountState::Unmounted => MountState::Unmounted,
        };
        return Ok(());
    }

    pub fn toggle_mount(&mut self) -> error::Result<()> {
        match self.state {
            MountState::Unmounted => self.mount(),
            MountState::Mounted(_) => self.unmount(),
        }
    }
}

impl render::Render for Iso {
    fn render(&self, window: &Window) -> error::Result<bitmap::Bitmap> {
        let mut base = bitmap::Bitmap::new(10, 1);
        base.blit(
            &font::render_text(
                self.path
                    .file_name()
                    .expect("iso has no name")
                    .to_string_lossy(),
            ),
            (16, 0),
        );
        match self.state {
            MountState::Mounted(_) => {
                base.blit(&bitmap::Bitmap::from_slice(font::SQUARE), (10, 0));
            }
            _ => (),
        };
        if window.focus {
            base.blit(&bitmap::Bitmap::from_slice(font::ARROW), (0, 0));
        }
        Ok(base)
    }
}

impl input::Input for Iso {
    fn on_event(
        &mut self,
        event: &controller::Event,
    ) -> error::Result<(bool, Vec<action::Action>)> {
        match *event {
            controller::Event::Select => {
                Ok((true, vec![action::Action::ToggleIsoMount(self.window)]))
            }
            _ => Ok((false, vec![])),
        }
    }

    fn do_action(
        &mut self,
        disp: &mut DisplayManager,
        action: &action::Action,
    ) -> error::Result<(bool, Vec<action::Action>)> {
        match *action {
            action::Action::ToggleIsoMount(id) if id == self.window => {
                self.toggle_mount()?;
                Ok((true, vec![]))
            }
            _ => Ok((false, vec![])),
        }
    }
}

impl Widget for Iso {
    fn mut_children(&mut self) -> Vec<&mut Widget> {
        vec![]
    }

    fn children(&self) -> Vec<&Widget> {
        vec![]
    }

    fn windowid(&self) -> WindowId {
        self.window
    }
}

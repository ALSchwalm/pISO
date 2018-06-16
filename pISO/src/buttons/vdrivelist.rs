use action;
use bitmap;
use buttons::back;
use controller;
use display;
use displaymanager::{DisplayManager, Position, Widget, Window, WindowId};
use error;
use font;
use input;
use lvm;
use render;
use state;
use vdrive;

struct DriveListItem {
    window: WindowId,
    onselect: fn(&str) -> action::Action,
    ismarked: fn(vdrive::PersistVDriveState) -> bool,
    name: String,
    autoclose: bool,
    listwindow: WindowId,
}

impl DriveListItem {
    fn new(
        disp: &mut DisplayManager,
        name: String,
        onselect: fn(&str) -> action::Action,
        ismarked: fn(vdrive::PersistVDriveState) -> bool,
        listwindow: WindowId,
        autoclose: bool,
    ) -> error::Result<DriveListItem> {
        let our_window = disp.add_child(Position::Normal)?;
        Ok(DriveListItem {
            window: our_window,
            onselect: onselect,
            ismarked: ismarked,
            name: name,
            listwindow: listwindow,
            autoclose: autoclose,
        })
    }
}

impl render::Render for DriveListItem {
    fn render(&self, _manager: &DisplayManager, window: &Window) -> error::Result<bitmap::Bitmap> {
        let mut base = bitmap::Bitmap::new(10, 1);
        base.blit(&font::render_text(&self.name), (12, 0));
        if window.focus {
            base.blit(&bitmap::Bitmap::from_slice(font::ARROW), (0, 0));
        }

        if (self.ismarked)(state::PERSISTENT_STATE
            .lock()
            .expect("Failed to lock state")
            .get(&self.name)?)
        {
            base.blit(&bitmap::Bitmap::from_slice(font::SQUARE), (6, 0));
        }
        Ok(base)
    }
}

impl input::Input for DriveListItem {
    fn on_event(
        &mut self,
        event: &controller::Event,
    ) -> error::Result<(bool, Vec<action::Action>)> {
        match *event {
            controller::Event::Select => {
                let mut actions = vec![(self.onselect)(&self.name)];
                if self.autoclose {
                    actions.push(action::Action::CloseVDriveList(self.listwindow));
                }
                Ok((true, actions))
            }
            _ => Ok((false, vec![])),
        }
    }
}

impl state::State for DriveListItem {}

impl Widget for DriveListItem {
    fn windowid(&self) -> WindowId {
        self.window
    }
}

struct DriveListSelector {
    window: WindowId,
    drives: Vec<DriveListItem>,
    backbutton: back::BackButton,
}

impl DriveListSelector {
    fn new(
        disp: &mut DisplayManager,
        parent: WindowId,
        vg: lvm::VolumeGroup,
        onselect: fn(&str) -> action::Action,
        ismarked: fn(vdrive::PersistVDriveState) -> bool,
        autoclose: bool,
    ) -> error::Result<DriveListSelector> {
        let our_window = disp.add_child(Position::Fixed(0, 0))?;
        let mut drives = vec![];
        for volume in vg.volumes()?.into_iter() {
            drives.push(DriveListItem::new(
                disp,
                volume.name,
                onselect.clone(),
                ismarked.clone(),
                parent,
                autoclose,
            )?)
        }
        let back = back::BackButton::new(disp, action::Action::CloseVDriveList(parent))?;
        if drives.len() > 0 {
            // Focus the first drive
            drives.iter().next().map(|drive| {
                disp.shift_focus(drive as &Widget);
            });
        } else {
            disp.shift_focus(&back);
        }
        Ok(DriveListSelector {
            window: our_window,
            drives: drives,
            backbutton: back,
        })
    }
}

impl render::Render for DriveListSelector {
    fn render(&self, _manager: &DisplayManager, _window: &Window) -> error::Result<bitmap::Bitmap> {
        Ok(bitmap::Bitmap::new(
            display::DISPLAY_WIDTH,
            display::DISPLAY_HEIGHT,
        ))
    }
}

impl input::Input for DriveListSelector {}

impl state::State for DriveListSelector {}

impl Widget for DriveListSelector {
    fn mut_children(&mut self) -> Vec<&mut Widget> {
        let mut children = self.drives
            .iter_mut()
            .map(|vdrive| vdrive as &mut Widget)
            .collect::<Vec<&mut Widget>>();
        children.push(&mut self.backbutton as &mut Widget);
        children
    }

    fn children(&self) -> Vec<&Widget> {
        let mut children = self.drives
            .iter()
            .map(|vdrive| vdrive as &Widget)
            .collect::<Vec<&Widget>>();
        children.push(&self.backbutton as &Widget);
        children
    }

    fn windowid(&self) -> WindowId {
        self.window
    }
}

enum DriveListState {
    Open(DriveListSelector),
    Closed,
}

pub struct DriveList {
    name: &'static str,
    window: WindowId,
    vg: lvm::VolumeGroup,
    state: DriveListState,
    onselect: fn(&str) -> action::Action,
    ismarked: fn(vdrive::PersistVDriveState) -> bool,
    autoclose: bool,
}

impl DriveList {
    pub fn new(
        disp: &mut DisplayManager,
        name: &'static str,
        vg: lvm::VolumeGroup,
        onselect: fn(&str) -> action::Action,
        ismarked: fn(vdrive::PersistVDriveState) -> bool,
        autoclose: bool,
    ) -> error::Result<DriveList> {
        let our_window = disp.add_child(Position::Normal)?;
        Ok(DriveList {
            window: our_window,
            name: name,
            vg: vg,
            state: DriveListState::Closed,
            onselect: onselect,
            ismarked: ismarked,
            autoclose: autoclose,
        })
    }
}

impl render::Render for DriveList {
    fn render(&self, _manager: &DisplayManager, window: &Window) -> error::Result<bitmap::Bitmap> {
        let mut base = bitmap::Bitmap::new(10, 1);
        base.blit(&font::render_text(self.name), (16, 0));
        if window.focus {
            base.blit(&bitmap::Bitmap::from_slice(font::ARROW), (0, 0));
        }
        Ok(base)
    }
}

impl input::Input for DriveList {
    fn on_event(
        &mut self,
        event: &controller::Event,
    ) -> error::Result<(bool, Vec<action::Action>)> {
        match *event {
            controller::Event::Select => {
                Ok((true, vec![action::Action::OpenVDriveList(self.window)]))
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
            action::Action::OpenVDriveList(id) if id == self.window => {
                self.state = DriveListState::Open(DriveListSelector::new(
                    disp,
                    self.window,
                    self.vg.clone(),
                    self.onselect.clone(),
                    self.ismarked.clone(),
                    self.autoclose,
                )?);
                Ok((true, vec![]))
            }
            action::Action::CloseVDriveList(id) if id == self.window => {
                self.state = DriveListState::Closed;
                disp.shift_focus(self);
                Ok((true, vec![]))
            }
            _ => Ok((false, vec![])),
        }
    }
}

impl state::State for DriveList {}

impl Widget for DriveList {
    fn mut_children(&mut self) -> Vec<&mut Widget> {
        match self.state {
            DriveListState::Open(ref mut selector) => vec![selector],
            DriveListState::Closed => vec![],
        }
    }

    fn children(&self) -> Vec<&Widget> {
        match self.state {
            DriveListState::Open(ref selector) => vec![selector],
            DriveListState::Closed => vec![],
        }
    }

    fn windowid(&self) -> WindowId {
        self.window
    }
}

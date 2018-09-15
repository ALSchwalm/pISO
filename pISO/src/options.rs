use action;
use buttons;
use bitmap;
use controller;
use displaymanager::{DisplayManager, Position, Widget, Window, WindowId};
use error;
use font;
use input;
use lvm;
use render;
use state;

pub struct Options {
    window: WindowId,
    open: bool,
    readonly: buttons::vdrivelist::DriveList,
    removable: buttons::vdrivelist::DriveList,
    delete: buttons::vdrivelist::DriveList,
    snapshot: buttons::vdrivelist::DriveList,
}

impl Options {
    pub fn new(disp: &mut DisplayManager, vg: &lvm::VolumeGroup) -> error::Result<Options> {
        let our_window = disp.add_child(Position::Normal)?;

        let readonly = buttons::vdrivelist::DriveList::new(
            disp,
            "Make Read-Only",
            vg.clone(),
            |drive| action::Action::ToggleDriveReadOnly(drive.to_string()),
            |state| state.readonly,
            false,
        )?;

        let removable = buttons::vdrivelist::DriveList::new(
            disp,
            "Make Nonremovable",
            vg.clone(),
            |drive| action::Action::ToggleDriveNonRemovable(drive.to_string()),
            |state| !state.removable,
            false,
        )?;

        let delete = buttons::vdrivelist::DriveList::new(
            disp,
            "Delete Drive",
            vg.clone(),
            |drive| action::Action::DeleteDrive(drive.to_string()),
            |_| false,
            true,
        )?;

        let snapshot = buttons::vdrivelist::DriveList::new(
            disp,
            "Snapshot Drive",
            vg.clone(),
            |drive| action::Action::SnapshotDrive(drive.to_string()),
            |_| false,
            true,
        )?;

        Ok(Options {
            window: our_window,
            open: false,
            readonly: readonly,
            removable: removable,
            delete: delete,
            snapshot: snapshot,
        })
    }
}

impl render::Render for Options {
    fn render(&self, _manager: &DisplayManager, window: &Window) -> error::Result<bitmap::Bitmap> {
        let mut base = bitmap::Bitmap::new(10, 1);
        base.blit(&font::render_text("Options"), (12, 0));
        if window.focus {
            base.blit(&bitmap::Bitmap::from_slice(font::ARROW), (0, 0));
        }
        Ok(base)
    }
}

impl input::Input for Options {
    fn on_event(
        &mut self,
        event: &controller::Event,
    ) -> error::Result<(bool, Vec<action::Action>)> {
        match *event {
            controller::Event::Select => {
                self.open = !self.open;
                Ok((true, vec![]))
            }
            _ => Ok((false, vec![])),
        }
    }
}

impl state::State for Options {}

impl Widget for Options {
    fn mut_children(&mut self) -> Vec<&mut Widget> {
        if self.open {
            vec![
                &mut self.readonly as &mut Widget,
                &mut self.removable as &mut Widget,
                &mut self.snapshot as &mut Widget,
                &mut self.delete as &mut Widget,
            ]
        } else {
            vec![]
        }
    }

    fn children(&self) -> Vec<&Widget> {
        if self.open {
            vec![
                &self.readonly as &Widget,
                &self.removable as &Widget,
                &self.snapshot as &Widget,
                &self.delete as &Widget,
            ]
        } else {
            vec![]
        }
    }

    fn windowid(&self) -> WindowId {
        self.window
    }
}

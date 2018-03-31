use action;
use bitmap;
use controller;
use displaymanager::{DisplayManager, Position, Widget, Window, WindowId};
use error::Result;
use input;
use lvm;
use newdrive;
use usb;
use std::sync::{Arc, Mutex};
use render;
use vdrive;

pub struct PIso {
    pub drives: Vec<vdrive::VirtualDrive>,
    newdrive: newdrive::NewDrive,
    usb: Arc<Mutex<usb::UsbGadget>>,
    vg: lvm::VolumeGroup,
    window: WindowId,
}

impl PIso {
    pub fn new(disp: Arc<Mutex<DisplayManager>>, usb: Arc<Mutex<usb::UsbGadget>>) -> Result<PIso> {
        let mut manager = disp.lock()?;
        let root = manager.root();
        let window = manager.add_child(root, Position::Normal)?;

        let vg = lvm::VolumeGroup::from_path("/dev/VolGroup00")?;
        let drives = Self::build_drives_from_vg(window, &mut manager, &vg, &usb)?;
        let ndrive = newdrive::NewDrive::new(&mut manager, window)?;

        if drives.len() > 0 {
            // Focus the first drive
            drives.iter().next().map(|drive| {
                manager.shift_focus(drive.window);
            });
        } else {
            manager.shift_focus(ndrive.window);
        }

        Ok(PIso {
            drives: drives,
            newdrive: ndrive,
            usb: usb,
            vg: vg,
            window: window,
        })
    }

    fn build_drives_from_vg(
        window: WindowId,
        disp: &mut DisplayManager,
        vg: &lvm::VolumeGroup,
        usb: &Arc<Mutex<usb::UsbGadget>>,
    ) -> Result<Vec<vdrive::VirtualDrive>> {
        let mut drives: Vec<vdrive::VirtualDrive> = vec![];
        for vol in vg.volumes()?.into_iter() {
            drives.push(vdrive::VirtualDrive::new(window, disp, usb.clone(), vol)?)
        }
        Ok(drives)
    }

    fn add_drive(&mut self, disp: &mut DisplayManager, size: u64) -> Result<&vdrive::VirtualDrive> {
        let volume = self.vg
            .create_volume(&format!("Drive{}", self.drives.len()), size)?;
        let vdrive = vdrive::VirtualDrive::new(self.window, disp, self.usb.clone(), volume)?;
        self.drives.push(vdrive);

        Ok(self.drives
            .last()
            .expect("vdrive was somehow empty after push"))
    }
}

impl render::Render for PIso {
    fn render(&self, window: &Window) -> Result<bitmap::Bitmap> {
        Ok(bitmap::Bitmap::new(0, 0))
    }
}

impl input::Input for PIso {
    fn on_event(&mut self, event: &controller::Event) -> (bool, Vec<action::Action>) {
        (false, vec![])
    }

    fn do_action(&mut self, disp: &mut DisplayManager, action: &action::Action) -> Result<bool> {
        match *action {
            action::Action::CreateDrive(size) => {
                self.add_drive(disp, size)?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

impl Widget for PIso {
    fn mut_children(&mut self) -> Vec<&mut Widget> {
        let mut children = self.drives
            .iter_mut()
            .map(|vdrive| vdrive as &mut Widget)
            .collect::<Vec<&mut Widget>>();
        children.push(&mut self.newdrive as &mut Widget);
        children
    }

    fn children(&self) -> Vec<&Widget> {
        let mut children = self.drives
            .iter()
            .map(|vdrive| vdrive as &Widget)
            .collect::<Vec<&Widget>>();
        children.push(&self.newdrive as &Widget);
        children
    }

    fn windowid(&self) -> WindowId {
        self.window
    }
}

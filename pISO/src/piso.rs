use bitmap;
use displaymanager::{DisplayManager, Position, Widget, Window, WindowId};
use error::Result;
use font;
use input;
use lvm;
use usb;
use std::sync::{Arc, Mutex};
use render;
use vdrive;

pub struct PIso {
    pub drives: Vec<vdrive::VirtualDrive>,
    usb: Arc<Mutex<usb::UsbGadget>>,
    disp: Arc<Mutex<DisplayManager>>,
    vg: lvm::VolumeGroup,
    window: WindowId,
}

impl PIso {
    pub fn new(disp: Arc<Mutex<DisplayManager>>, usb: Arc<Mutex<usb::UsbGadget>>) -> Result<PIso> {
        let window = {
            let mut manager = disp.lock()?;
            let root = manager.root();
            manager.add_child(root, Position::Normal)?
        };
        let vg = lvm::VolumeGroup::from_path("/dev/VolGroup00")?;
        let drives = Self::build_drives_from_vg(window, &disp, &vg, &usb)?;

        Ok(PIso {
            drives: drives,
            usb: usb,
            vg: vg,
            window: window,
            disp: disp.clone(),
        })
    }

    fn build_drives_from_vg(
        window: WindowId,
        disp: &Arc<Mutex<DisplayManager>>,
        vg: &lvm::VolumeGroup,
        usb: &Arc<Mutex<usb::UsbGadget>>,
    ) -> Result<Vec<vdrive::VirtualDrive>> {
        let mut drives: Vec<vdrive::VirtualDrive> = vec![];
        for vol in vg.volumes()?.into_iter() {
            drives.push(vdrive::VirtualDrive::new(
                window,
                disp.clone(),
                usb.clone(),
                vol,
            )?)
        }
        Ok(drives)
    }

    pub fn add_drive(&mut self, size: u64) -> Result<&vdrive::VirtualDrive> {
        let volume = self.vg
            .create_volume(&format!("Drive{}", self.drives.len()), size)?;
        let vdrive =
            vdrive::VirtualDrive::new(self.window, self.disp.clone(), self.usb.clone(), volume)?;
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
    fn on_up(&mut self) -> bool {
        true
    }
    fn on_down(&mut self) -> bool {
        true
    }
    fn on_select(&mut self) -> bool {
        true
    }
}

impl Widget for PIso {
    fn children(&self) -> Vec<&Widget> {
        self.drives.iter().map(|vdrive| vdrive as &Widget).collect()
    }

    fn windowid(&self) -> WindowId {
        self.window
    }
}

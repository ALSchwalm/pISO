use error::Result;
use lvm;
use usb;
use std::sync::{Arc, Mutex};
use vdrive;

pub struct PIso {
    drives: Vec<vdrive::VirtualDrive>,
    usb: Arc<Mutex<usb::UsbGadget>>,
    vg: lvm::VolumeGroup,
}

impl PIso {
    fn new(usb: Arc<Mutex<usb::UsbGadget>>) -> Result<PIso> {
        let vg = lvm::VolumeGroup::from_path("/dev/VolGroup00")?;
        let drives = Self::build_drives_from_vg(&vg, &usb)?;

        Ok(PIso {
            drives: drives,
            usb: usb,
            vg: vg,
        })
    }

    fn build_drives_from_vg(
        vg: &lvm::VolumeGroup,
        usb: &Arc<Mutex<usb::UsbGadget>>,
    ) -> Result<Vec<vdrive::VirtualDrive>> {
        let mut drives = vec![];
        for vol in vg.volumes()?.into_iter() {
            drives.push(vdrive::VirtualDrive::new(vol, usb.clone())?)
        }
        Ok(drives)
    }
}

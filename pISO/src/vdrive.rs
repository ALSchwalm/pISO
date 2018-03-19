use error::{ErrorKind, Result};
use usb;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

enum MountState {
    Unmounted,
    Internal,
    External(usb::StorageID),
}

struct VirtualDrive {
    name: String,
    state: MountState,
    usb: Arc<Mutex<usb::UsbGadget>>,
    path: PathBuf,
}

impl VirtualDrive {
    fn new(name: String, usb: Arc<Mutex<usb::UsbGadget>>) -> Result<VirtualDrive> {
        //TODO: create the volume
        Ok(VirtualDrive {
            path: PathBuf::from(format!("/dev/VolGroup00/{}", name)),
            name: name,
            state: MountState::Unmounted,
            usb: usb,
        })
    }

    fn from<P>(path: P, usb: Arc<Mutex<usb::UsbGadget>>) -> Result<VirtualDrive>
    where
        P: AsRef<Path>,
    {
        Ok(VirtualDrive {
            path: path.as_ref().to_path_buf(),
            name: path.as_ref()
                .file_name()
                .ok_or(ErrorKind::Msg("vdrive path has no filename".into()))?
                .to_string_lossy()
                .into(),
            state: MountState::Unmounted,
            usb: usb,
        })
    }

    fn mount_external(&mut self) -> Result<()> {
        let id = self.usb
            .lock()
            .map_err(|_| "Failed to lock usb mutex")?
            .export_file(&self.path, false)?;
        self.state = MountState::External(id);
        Ok(())
    }

    fn unmount_external(&mut self) -> Result<()> {
        match self.state {
            MountState::Unmounted => Ok(()),
            MountState::Internal => {
                Err("Attempt to unmount_external while mounted internal".into())
            }
            MountState::External(ref id) => self.usb
                .lock()
                .map_err(|_| "Failed to lock usb mutex")?
                .unexport_file(id),
        }
    }
}

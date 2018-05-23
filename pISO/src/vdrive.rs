use action;
use bitmap;
use controller;
use displaymanager::{DisplayManager, Position, Widget, Window, WindowId};
use error::{ErrorKind, Result, ResultExt};
use font;
use input;
use iso;
use lvm;
use usb;
use utils;
use render;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

const VDRIVE_MOUNT_ROOT: &str = "/mnt";

pub struct MountInfo {
    pub loopback_path: PathBuf,
    pub part_mount_paths: Vec<PathBuf>,
    pub isos: Vec<iso::Iso>,
}

pub enum MountState {
    Unmounted,
    Internal(MountInfo),
    External(usb::StorageID),
}

pub struct VirtualDrive {
    pub state: MountState,
    pub usb: Arc<Mutex<usb::UsbGadget>>,
    pub volume: lvm::LogicalVolume,
    pub window: WindowId,
}

impl VirtualDrive {
    pub fn new(
        disp: &mut DisplayManager,
        usb: Arc<Mutex<usb::UsbGadget>>,
        volume: lvm::LogicalVolume,
    ) -> Result<VirtualDrive> {
        let our_window = disp.add_child(Position::Normal)?;
        Ok(VirtualDrive {
            window: our_window,
            state: MountState::Unmounted,
            usb: usb,
            volume: volume,
        })
    }

    pub fn name(&self) -> &str {
        &self.volume.name
    }

    pub fn mount_external(&mut self) -> Result<()> {
        match self.state {
            MountState::External(_) => Ok(()),
            MountState::Unmounted => {
                let id = self.usb
                    .lock()?
                    .export_file(&self.volume.path, false)
                    .chain_err(|| "failed to mount drive external")?;
                self.state = MountState::External(id);
                Ok(())
            }
            MountState::Internal(_) => {
                Err("Attempt to mount_external while mounted internal".into())
            }
        }
    }

    pub fn unmount_external(&mut self) -> Result<()> {
        match self.state {
            MountState::Unmounted => {}
            MountState::Internal(_) => {
                return Err("Attempt to unmount_external while mounted internal".into());
            }
            MountState::External(ref id) => {
                self.usb
                    .lock()?
                    .unexport_file(id)
                    .chain_err(|| "failed to unmount external")?;
            }
        }
        self.state = MountState::Unmounted;
        Ok(())
    }

    fn mount_partition<P1, P2>(&self, device: P1, target: P2) -> Result<()>
    where
        P1: AsRef<Path>,
        P2: AsRef<Path>,
    {
        let mounters = &["mount", "mount.exfat"];
        for mounter in mounters {
            let fsmount = utils::run_check_output(mounter, &[device.as_ref(), target.as_ref()]);
            if fsmount.is_ok() {
                return Ok(());
            }
        }
        Err(format!(
            "Failed to mount: {} to {}",
            device.as_ref().display(),
            target.as_ref().display()
        ).into())
    }

    pub fn mount_internal(&mut self, disp: &mut DisplayManager) -> Result<()> {
        match self.state {
            MountState::Unmounted => {
                let volume_path = &self.volume.path.to_string_lossy();
                let loopback_path =
                    PathBuf::from(utils::run_check_output("losetup", &["-f"])?.trim_right());
                let loopback_name: String = loopback_path
                    .file_name()
                    .ok_or(ErrorKind::Msg("loopback path has no file name".into()))?
                    .to_string_lossy()
                    .into();

                utils::run_check_output("losetup", &["-fP", volume_path])?;

                let mut mounted_partitions = vec![];
                let mut isos = vec![];
                for entry in fs::read_dir("/dev")? {
                    let entry = entry?;
                    if entry
                        .file_name()
                        .to_string_lossy()
                        .starts_with(&loopback_name)
                    {
                        let mount_point = Path::new(VDRIVE_MOUNT_ROOT).join(entry.file_name());
                        fs::create_dir_all(&mount_point)?;
                        if self.mount_partition(&entry.path(), &mount_point).is_ok() {
                            mounted_partitions.push(mount_point.to_path_buf());

                            let isopath = mount_point.join("ISOS");
                            if isopath.exists() {
                                for iso in fs::read_dir(isopath)? {
                                    let iso = iso?;
                                    isos.push(iso::Iso::new(disp, self.usb.clone(), iso.path())?);
                                }
                            }
                        }
                    }
                }
                self.state = MountState::Internal(MountInfo {
                    part_mount_paths: mounted_partitions,
                    isos: isos,
                    loopback_path: loopback_path.to_path_buf(),
                });
                Ok(())
            }
            MountState::Internal(_) => Ok(()),
            MountState::External(_) => {
                Err("Attempt to mount_internal while mounted external".into())
            }
        }
    }

    pub fn unmount_internal(&mut self) -> Result<()> {
        match self.state {
            MountState::Unmounted => {}
            MountState::Internal(ref mut info) => {
                for iso in info.isos.iter_mut() {
                    iso.unmount()?;
                }
                for part in info.part_mount_paths.iter() {
                    utils::run_check_output("umount", &[&part])?;
                }
                utils::run_check_output("losetup", &["-d", &info.loopback_path.to_string_lossy()])?;
            }
            MountState::External(_) => {
                return Err("Attempt to unmount_internal while mounted external".into());
            }
        };
        self.state = MountState::Unmounted;
        Ok(())
    }

    pub fn toggle_mount(&mut self, disp: &mut DisplayManager) -> Result<()> {
        match self.state {
            // For now, just switch to external if unmounted
            MountState::Unmounted => self.mount_external(),
            MountState::Internal(_) => {
                self.unmount_internal()?;
                self.mount_external()
            }
            MountState::External(_) => {
                self.unmount_external()?;
                self.mount_internal(disp)
            }
        }
    }
}

impl render::Render for VirtualDrive {
    fn render(&self, _manager: &DisplayManager, window: &Window) -> Result<bitmap::Bitmap> {
        let mut base = bitmap::Bitmap::new(10, 1);
        base.blit(&font::render_text(self.name()), (12, 0));
        match self.state {
            MountState::External(_) => {
                base.blit(&bitmap::Bitmap::from_slice(font::SQUARE), (6, 0));
            }
            _ => (),
        };
        if window.focus {
            base.blit(&bitmap::Bitmap::from_slice(font::ARROW), (0, 0));
        }
        Ok(base)
    }
}

impl input::Input for VirtualDrive {
    fn on_event(&mut self, event: &controller::Event) -> Result<(bool, Vec<action::Action>)> {
        match *event {
            controller::Event::Select => {
                Ok((true, vec![action::Action::ToggleVDriveMount(self.window)]))
            }
            _ => Ok((false, vec![])),
        }
    }

    fn do_action(
        &mut self,
        disp: &mut DisplayManager,
        action: &action::Action,
    ) -> Result<(bool, Vec<action::Action>)> {
        match *action {
            action::Action::ToggleVDriveMount(id) if id == self.window => {
                self.toggle_mount(disp)?;
                Ok((true, vec![]))
            }
            _ => Ok((false, vec![])),
        }
    }
}

impl Widget for VirtualDrive {
    fn mut_children(&mut self) -> Vec<&mut Widget> {
        match self.state {
            MountState::Internal(ref mut info) => {
                info.isos.iter_mut().map(|iso| iso as &mut Widget).collect()
            }
            _ => vec![],
        }
    }

    fn children(&self) -> Vec<&Widget> {
        match self.state {
            MountState::Internal(ref info) => info.isos.iter().map(|iso| iso as &Widget).collect(),
            _ => vec![],
        }
    }

    fn windowid(&self) -> WindowId {
        self.window
    }
}

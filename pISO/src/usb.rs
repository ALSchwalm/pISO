use error::{ErrorKind, Result, ResultExt};
use std::fs::{create_dir_all, read_dir, remove_file, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::io::Write;
use std::os::unix::fs::symlink;
use std::time::{Duration, Instant};
use std::thread;
use utils::wait_for_path;

pub struct GadgetConfig {
    pub vendor_id: &'static str,
    pub product_id: &'static str,
    pub device_bcd: &'static str,
    pub usb_bcd: &'static str,

    // The serial number will vary from unit to unit
    pub serial_number: String,

    pub manufacturer: &'static str,
    pub product: &'static str,
    pub max_power: &'static str,
    pub configuration: &'static str,
}

pub struct UsbGadget {
    config: GadgetConfig,
    root: PathBuf,
    current_id: u64,
}

pub struct StorageID(u64);

impl UsbGadget {
    pub fn new<P>(root: P, config: GadgetConfig) -> Result<UsbGadget>
    where
        P: AsRef<Path>,
    {
        // Wait for the UDC to exist in sysfs
        wait_for_path(root.as_ref().join("UDC"), Duration::from_millis(1500))?;

        let mut gadget = UsbGadget {
            config: config,
            root: PathBuf::from(root.as_ref()),
            current_id: 0,
        };

        gadget
            .configure()
            .chain_err(|| "Failed to configure usb gadget")?;

        Ok(gadget)
    }

    fn configure(&mut self) -> Result<()> {
        File::create(self.root.join("idVendor"))?.write_all(self.config.vendor_id.as_bytes())?;
        File::create(self.root.join("idProduct"))?.write_all(self.config.product_id.as_bytes())?;
        File::create(self.root.join("bcdDevice"))?.write_all(self.config.device_bcd.as_bytes())?;
        File::create(self.root.join("bcdUSB"))?.write_all(self.config.usb_bcd.as_bytes())?;

        create_dir_all(self.root.join("strings/0x409/"))?;

        File::create(self.root.join("strings/0x409/serialnumber"))?
            .write_all(self.config.serial_number.as_bytes())?;
        File::create(self.root.join("strings/0x409/manufacturer"))?
            .write_all(self.config.manufacturer.as_bytes())?;
        File::create(self.root.join("strings/0x409/product"))?
            .write_all(self.config.product.as_bytes())?;

        create_dir_all(self.root.join("configs/c.1/strings/0x409"))?;

        File::create(self.root.join("configs/c.1/MaxPower"))?
            .write_all(self.config.max_power.as_bytes())?;
        File::create(self.root.join("configs/c.1/strings/0x409/configuration"))?
            .write_all(self.config.configuration.as_bytes())?;
        Ok(())
    }

    fn activate_udc_if_ready(&mut self) -> Result<()> {
        let mut udcs = read_dir("/sys/class/udc")?;
        let udc = udcs.next()
            .ok_or(ErrorKind::Msg("No available udc".into()))??
            .file_name();

        let functions = read_dir(self.root.join("configs/c.1"))?;
        for entry in functions {
            let entry = entry?;
            if entry.file_type()?.is_symlink() {
                println!("Activating UDC");
                File::create(self.root.join("UDC"))?.write_all(udc.to_string_lossy().as_bytes())?;

                assert!(
                    self.is_udc_active()
                        .chain_err(|| "failed to check if udc is active")?,
                    "UDC was not active after activate"
                );
                break;
            }
        }
        Ok(())
    }

    fn is_udc_active(&mut self) -> Result<bool> {
        let udc_path = self.root.join("UDC");
        let mut file = File::open(&udc_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(contents != "\n")
    }

    fn deactivate_udc(&mut self) -> Result<()> {
        let udc_path = self.root.join("UDC");

        // Only deactivate if it is currently active
        if self.is_udc_active()
            .chain_err(|| "failed to check if udc is active")?
        {
            println!("Deactivating UDC: {}", udc_path.display());
            File::create(udc_path)?.write_all(b"\n")?;
        }

        assert!(
            !self.is_udc_active()
                .chain_err(|| "failed to check if udc is active")?,
            "UDC was still active after deactivate"
        );
        Ok(())
    }

    //TODO: should this be derived from the file in some way?
    fn new_storage_id(&mut self) -> StorageID {
        let new_id = StorageID(self.current_id);
        self.current_id += 1;
        new_id
    }

    pub fn export_file<P>(&mut self, path: P, cdrom: bool) -> Result<StorageID>
    where
        P: AsRef<Path>,
    {
        self.deactivate_udc()
            .chain_err(|| "failed to deactivate UDC")?;
        let id = self.new_storage_id();

        let storage_root = self.root
            .join(format!("functions/mass_storage.{}/lun.0", id.0));
        create_dir_all(&storage_root)?;

        File::create(
            self.root
                .join(format!("functions/mass_storage.{}/stall", id.0)),
        )?.write_all(b"1")?;

        File::create(storage_root.join("cdrom"))?.write_all((cdrom as i32).to_string().as_bytes())?;

        // This seems like a bug. If 'ro' has already been set, you cannot
        // change it (resource busy) even if UDC is inactive and the
        // config is removed. For now just suppress the error.
        File::create(storage_root.join("ro"))?.write_all((cdrom as i32).to_string().as_bytes())?;

        File::create(storage_root.join("file"))?
            .write_all(path.as_ref().to_string_lossy().as_bytes())?;

        symlink(
            self.root.join(format!("functions/mass_storage.{}", id.0)),
            self.root.join(format!("configs/c.1/mass_storage.{}", id.0)),
        )?;

        self.activate_udc_if_ready()
            .chain_err(|| "failed to activate UDC")?;
        Ok(id)
    }

    pub fn unexport_file(&mut self, id: &StorageID) -> Result<()> {
        self.deactivate_udc()
            .chain_err(|| "failed to deactivate UDC")?;

        remove_file(self.root.join(format!("configs/c.1/mass_storage.{}", id.0)))?;

        self.activate_udc_if_ready()
            .chain_err(|| "failed to activate UDC")?;
        Ok(())
    }
}

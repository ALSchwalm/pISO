use error::Result;
use std::fs::File;
use std::io::Read;
use std::path::{Path,PathBuf};
use std::thread;
use std::time::{Duration, Instant};

pub struct GadgetConfig {
    pub vendor_id: u16,
    pub product_id: u16,
    pub device_bcd: u16,
    pub usb_bcd: u16,

    pub serial_number: &'static str,
    pub manufacturer: &'static str,
    pub productr: &'static str,

    pub max_power: u16,
    pub configuration: &'static str
}

pub struct UsbGadget {
    config: GadgetConfig,
    root: PathBuf,
}

impl UsbGadget {
    pub fn new<P>(root: P, config: GadgetConfig) -> Result<UsbGadget>
        where P: AsRef<Path> {

        // Wait for the UDC to exist in sysfs
        Self::wait_for_gadget(&root, Duration::from_millis(1500))?;

        let gadget = UsbGadget {
            config: config,
            root: PathBuf::from(root.as_ref())
        };

        Ok(gadget)
    }

    // fn configure(&mut self, config: GadgetConfig)

    fn wait_for_gadget<P>(path: P, total_wait: Duration) -> Result<()>
        where P: AsRef<Path>
    {
        let now = Instant::now();
        let wait = Duration::from_millis(50);
        let udc_path = path.as_ref().join("/UDC");

        while now.elapsed() < total_wait {
            if udc_path.exists() {
                return Ok(())
            }
            thread::sleep(wait);
        }
        Err("timeout while waiting for gadget".into())
    }

    fn activate_udc(&mut self) -> Result<()> {
        let mut file = File::open("/sys/class/udc")?;
        let mut udc = String::new();
        file.read_to_string(&mut udc)?;

        Ok(())
    }
}

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate derive_error_chain;
extern crate mio;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate spidev;
extern crate sysfs_gpio;

use std::thread;

mod bitmap;
mod controller;
mod display;
mod displaymanager;
mod error;
mod font;
mod input;
mod lvm;
mod piso;
mod render;
mod usb;
mod utils;
mod vdrive;

use error::ResultExt;
use std::sync::{Arc, Mutex};

quick_main!(run);

fn run() -> error::Result<()> {
    let mut manager = displaymanager::DisplayManager::new()?;

    let mut gadget = Arc::new(Mutex::new(usb::UsbGadget::new(
        "/sys/kernel/config/usb_gadget/g1",
        usb::GadgetConfig {
            vendor_id: "0x1d6b",
            product_id: "0x0104",
            device_bcd: "0x0100",
            usb_bcd: "0x0200",

            // Pull the serial number from the Pi's proc/cpuinfo
            serial_number: utils::run_check_output(
                "awk",
                &["/Serial/{print $3}", "/proc/cpuinfo"],
            )?,
            manufacturer: "Adam Schwalm & James Tate",
            product: "pISO",

            max_power: "250",
            configuration: "Config 1",
        },
    )?));

    let mut piso = piso::PIso::new(manager.clone(), gadget)?;

    manager.lock()?.render(&piso)?;

    let mut controller = controller::Controller::new()?;
    controller.on_select(Box::new(move || {
        piso.add_drive(12 * 1024 * 1024);
        manager
            .lock()
            .unwrap()
            .render(&piso)
            .expect("Render failed");
        println!("select");
    }));
    controller.on_up(Box::new(|| {
        println!("up");
    }));
    controller.on_down(Box::new(|| {
        println!("down");
    }));

    controller.start().expect("controller failed");

    Ok(())
}

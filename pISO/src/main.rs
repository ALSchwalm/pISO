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
    println!("Building display manager");
    let mut manager = displaymanager::DisplayManager::new()?;

    println!("Building USB gadget");
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

    println!("Building pISO");
    let mut piso = piso::PIso::new(manager.clone(), gadget)?;

    println!("Rendering pISO");
    manager.lock()?.render(&piso)?;

    println!("Building controller");
    let mut controller = controller::Controller::new()?;
    controller.on_event(Box::new(move |event| {
        match event {
            controller::Event::Up => {}
            controller::Event::Down => {}
            controller::Event::Select => {
                piso.add_drive(12 * 1024 * 1024);
            }
        }

        let mut manager = manager.lock().unwrap();
        manager
            .on_event(&mut piso, &event)
            .expect("Event handling failed");

        manager.render(&piso).expect("Render failed");
    }));

    controller.start().expect("controller failed");

    Ok(())
}

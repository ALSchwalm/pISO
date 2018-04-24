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
extern crate toml;

use std::fs;
use std::thread;

mod action;
mod bitmap;
mod config;
mod controller;
mod display;
mod displaymanager;
mod error;
mod font;
mod input;
mod iso;
mod lvm;
mod newdrive;
mod piso;
mod render;
mod stats;
mod usb;
mod utils;
mod vdrive;
mod wifi;

use error::ResultExt;
use std::sync::{Arc, Mutex};
use std::io::Read;

quick_main!(run);

fn run() -> error::Result<()> {
    let mut f = fs::File::open("/boot/piso.config").expect("config file not found");
    let mut config_contents = String::new();
    f.read_to_string(&mut config_contents)
        .expect("unable to read config");
    let config: config::Config = toml::from_str(&config_contents)?;

    let display = display::LedDisplay::new()?;

    println!("Building display manager");
    let mut manager = displaymanager::DisplayManager::new(display)?;

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
    let mut piso = piso::PIso::new(manager.clone(), gadget, config)?;

    println!("Rendering pISO");
    manager.lock()?.render(&piso)?;

    println!("Building controller");
    let mut controller = controller::Controller::new()?;
    controller.on_event(Box::new(move |event| {
        let mut manager = manager.lock().unwrap();

        println!("Handling event: {:?}", event);
        let mut actions = manager
            .on_event(&mut piso, &event)
            .expect("Event handling failed");

        // Keep processing until all actions are finished
        while {
            println!("Doing actions: {:?}", actions);
            manager
                .do_actions(&mut piso, &mut actions)
                .expect("Doing actions failed");

            println!("Rendering");
            manager.render(&piso).expect("Render failed");
            actions.len() > 0
        } {}

        println!("Event loop finished");
    }));

    controller.start().expect("controller failed");

    Ok(())
}

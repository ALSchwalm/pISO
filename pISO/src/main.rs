#![allow(non_snake_case)]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate derive_error_chain;
#[macro_use]
extern crate lazy_static;
extern crate mio;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate spidev;
extern crate sysfs_gpio;
extern crate tar;
extern crate toml;

use std::fs;

mod action;
mod bitmap;
mod buttons;
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
mod options;
mod piso;
mod render;
mod state;
mod stats;
mod usb;
mod utils;
mod vdrive;
mod version;
mod wifi;

use error::ResultExt;
use error_chain::ChainedError;
use std::io::Read;
use std::path::Path;
use std::sync::{Arc, Mutex};

quick_main!(trap_error);

fn trap_error() -> error::Result<()> {
    let display = display::LedDisplay::new()?;

    println!("Building display manager");
    let mut manager = displaymanager::DisplayManager::new(display)?;

    let err = run(&mut manager);

    // Write the error to stdout and update the screen
    match err {
        Err(ref e) => {
            println!("{}", e.display_chain());

            let mut msg = bitmap::Bitmap::new(display::DISPLAY_WIDTH, display::DISPLAY_HEIGHT);
            msg.blit(&font::render_text("An error occurred."), (0, 0));
            msg.blit(&font::render_text("Please visit:"), (0, 14));
            msg.blit(&font::render_text("http://piso.support"), (0, 28));
            manager.display.update(msg)?;
        }
        _ => (),
    };

    // Build the error tarball
    let tarfile = fs::File::create("/boot/piso_debug.tar")?;
    let mut tarbuilder = tar::Builder::new(tarfile);

    // Don't add the config by default, because it will contain wifi passwords
    let files = vec!["/tmp/piso.log", "/boot/piso.state", "/tmp/messages"];
    for file in files {
        let p = Path::new(file);
        if p.exists() {
            tarbuilder.append_file(p.file_name().unwrap(), &mut fs::File::open(file)?)?;
        }
    }
    tarbuilder.finish()?;

    panic!("pISO terminated")
}

fn run(manager: &mut displaymanager::DisplayManager) -> error::Result<()> {
    let mut f = fs::File::open("/boot/piso.config").chain_err(|| "config file not found")?;
    let mut config_contents = String::new();
    f.read_to_string(&mut config_contents)
        .expect("unable to read config");
    let config: config::Config =
        toml::from_str(&config_contents).chain_err(|| "failed to parse config file")?;

    println!("Building USB gadget");
    let gadget = Arc::new(Mutex::new(usb::UsbGadget::new(
        "/sys/kernel/config/usb_gadget/g1",
        usb::GadgetConfig {
            vendor_id: "0x1209",  // pid.codes vendor id
            product_id: "0x0256", // pISO Hat product id
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
    let mut piso = piso::PIso::new(manager, gadget, &config)?;

    println!("Restoring State");
    state::PERSISTENT_STATE
        .lock()
        .expect("Failed to lock state")
        .load_state(&mut piso, manager)?;

    println!("Rendering pISO");
    manager.render(&piso)?;

    println!("Building controller");
    let controller = controller::Controller::new(&config)?;
    for event in controller {
        println!("Handling event: {:?}", event);
        let mut actions = manager
            .on_event(&mut piso, &event)
            .chain_err(|| "Event handling failed")?;

        // Keep processing until all actions are finished
        while {
            println!("Doing actions: {:?}", actions);
            manager
                .do_actions(&mut piso, &mut actions)
                .chain_err(|| "Doing actions failed")?;

            println!("Rendering");
            manager.render(&piso).chain_err(|| "Render failed")?;
            actions.len() > 0
        } {}
        println!("Event loop finished");

        println!("Saving state");
        state::PERSISTENT_STATE
            .lock()
            .expect("Failed to lock state")
            .save_state(&mut piso)?;

        // Some rendering pulls saved state from other widgets, so
        // do a final render to update those
        println!("Final Render");
        manager.render(&piso).chain_err(|| "Final render failed")?;
    }

    Ok(())
}

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
mod error;
mod font;
mod lvm;
mod piso;
mod usb;
mod utils;
mod vdrive;

use error::ResultExt;
use std::sync::{Arc, Mutex};

quick_main!(run);

fn run() -> error::Result<()> {
    let mut disp = display::Display::new().chain_err(|| "Failed to create display")?;
    disp.on().chain_err(|| "Failed to activate display")?;

    let bitmap = font::render_text("hello");
    disp.update(bitmap)?;

    let mut gadget = Arc::new(Mutex::new(usb::UsbGadget::new(
        "/sys/kernel/config/usb_gadget/g1",
        usb::GadgetConfig {
            vendor_id: "0x1d6b",
            product_id: "0x0104",
            device_bcd: "0x0100",
            usb_bcd: "0x0200",

            serial_number: "0000000000000000",
            manufacturer: "Adam Schwalm & James Tate",
            product: "pISO",

            max_power: "250",
            configuration: "Config 1",
        },
    )?));

    let mut vg = lvm::VolumeGroup::from_path("/dev/VolGroup00")?;
    let volume = vg.create_volume("Drive0", 12 * 1024 * 1024)?;
    let mut vdrive = vdrive::VirtualDrive::new(volume, gadget.clone())?;
    vdrive.mount_external()?;

    let mut controller = controller::Controller::new()?;
    controller.on_select(Box::new(move || {
        vdrive.unmount_external().expect("Unmount external failed");
        vdrive.mount_internal().expect("Mount internal failed");

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

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate derive_error_chain;
extern crate mio;
extern crate spidev;
extern crate sysfs_gpio;

use std::thread;

mod bitmap;
mod controller;
mod display;
mod error;
use error::ResultExt;
mod font;
mod usb;

quick_main!(run);

fn run() -> error::Result<()> {
    let mut disp = display::Display::new()
        .chain_err(|| "Failed to create display")?;
    disp.on().chain_err(|| "Failed to activate display")?;

    let bitmap = font::render_text("hello");
    disp.update(bitmap)?;

    let mut gadget = usb::UsbGadget::new("/sys/kernel/config/usb_gadget/g1",
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
                        })?;

    gadget.export_file("/usr/bin/perf", false)?;

    let mut controller = controller::Controller::new()?;
    controller.on_select(Box::new(||{
        println!("select");
    }));
    controller.on_up(Box::new(||{
        println!("up");
    }));
    controller.on_down(Box::new(||{
        println!("down");
    }));

    controller.start().expect("controller failed");

    Ok(())
}

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate derive_error_chain;
extern crate mio;
extern crate spidev;
extern crate sysfs_gpio;

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

    println!("Making controller");
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

    println!("starting controller");
    controller.start()?;

    Ok(())
}

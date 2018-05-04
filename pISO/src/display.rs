use bitmap::Bitmap;
use spidev::{SPI_MODE_0, Spidev, SpidevOptions};
use std::io::Write;
use std::thread;
use std::time;
use sysfs_gpio::{Direction, Pin};
use error;

#[allow(unused)]
enum SSD1306Command {
    // Constants
    I2CAddress = 0x3C, // 011110+SA0+RW - 0x3C or 0x3D
    SetContrast = 0x81,
    DisplayAllOnResume = 0xA4,
    DisplayAllOn = 0xA5,
    NormalDisplay = 0xA6,
    InvertDisplay = 0xA7,
    DisplayOff = 0xAE,
    DisplayOn = 0xAF,
    SetDisplayOffset = 0xD3,
    SetComPins = 0xDA,
    SetVComDetect = 0xDB,
    SetDisplayClockDiv = 0xD5,
    SetPrecharge = 0xD9,
    SetMultiplex = 0xA8,
    SetLowColumn = 0x00,
    SetHighColumn = 0x10,
    SetStartLine = 0x40,
    MemoryMode = 0x20,
    ColumnAddr = 0x21,
    PageAddr = 0x22,
    ComScanInc = 0xC0,
    ComScanDec = 0xC8,
    SegRemap = 0xA0,
    ChargePump = 0x8D,
    ExternalVCC = 0x1,
    SwitchcapVcc = 0x2,

    // Scrolling constants
    ActivateScroll = 0x2F,
    DeactivateScroll = 0x2E,
    SetVerticalScrollArea = 0xA3,
    RightHorizontalScroll = 0x26,
    LeftHorizontalScroll = 0x27,
    VerticalAndRightHorizontalScroll = 0x29,
    VerticalAndLeftHorizontalScroll = 0x2A,
}

impl Into<u8> for SSD1306Command {
    fn into(self) -> u8 {
        self as u8
    }
}

pub struct LedDisplay {
    inverted: bool,
    contents: Bitmap,
    dc_pin: Pin,
    rst_pin: Pin,
    bus: Spidev,
}

pub trait Display {
    fn on(&mut self) -> error::Result<()>;
    fn reset(&mut self) -> error::Result<()>;
    fn update(&mut self, bitmap: Bitmap) -> error::Result<()>;
    fn flip_display(&mut self);
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

impl LedDisplay {
    pub fn new() -> error::Result<Box<Display>> {
        let mut spi = Spidev::open("/dev/spidev0.0")?;
        let options = SpidevOptions::new()
            .bits_per_word(8)
            .max_speed_hz(8000000)
            .mode(SPI_MODE_0)
            .build();
        spi.configure(&options)?;

        let dc_pin = Pin::new(19);
        dc_pin.export()?;
        dc_pin.set_direction(Direction::Out)?;

        let rst_pin = Pin::new(25);
        rst_pin.export()?;
        rst_pin.set_direction(Direction::Out)?;

        Ok(Box::new(LedDisplay {
            inverted: true,
            contents: Bitmap::new(128, 64),
            dc_pin: dc_pin,
            rst_pin: rst_pin,
            bus: spi,
        }))
    }

    fn send_spi_command<Cmd>(&mut self, cmd: Cmd) -> error::Result<()>
    where
        Cmd: Into<u8>,
    {
        self.dc_pin.set_value(0)?;
        self.bus.write(&[cmd.into()])?;
        Ok(())
    }

    fn send_spi_data(&mut self, data: &[u8]) -> error::Result<()> {
        self.dc_pin.set_value(1)?;
        self.bus.write(data)?;
        Ok(())
    }
}

impl Display for LedDisplay {
    fn width(&self) -> usize {
        self.contents.first().map(|row| row.len()).unwrap_or(0)
    }

    fn height(&self) -> usize {
        self.contents.len()
    }

    fn on(&mut self) -> error::Result<()> {
        self.reset()?;

        self.send_spi_command(SSD1306Command::DisplayOff)?;
        self.send_spi_command(SSD1306Command::SetDisplayClockDiv)?;
        self.send_spi_command(0x80)?; // the suggested ratio 0x80
        self.send_spi_command(SSD1306Command::SetMultiplex)?;
        self.send_spi_command(0x3F)?;
        self.send_spi_command(SSD1306Command::SetDisplayOffset)?;
        self.send_spi_command(0x0)?; // no offset
        self.send_spi_command((SSD1306Command::SetStartLine as u8) | 0x0)?; // line #0
        self.send_spi_command(SSD1306Command::ChargePump)?;
        self.send_spi_command(0x14)?;
        self.send_spi_command(SSD1306Command::MemoryMode)?;
        self.send_spi_command(0x00)?; // 0x0 act like ks0108
        self.send_spi_command((SSD1306Command::SegRemap as u8) | 0x1)?;
        self.send_spi_command(SSD1306Command::ComScanDec)?;
        self.send_spi_command(SSD1306Command::SetComPins)?;
        self.send_spi_command(0x12)?;
        self.send_spi_command(SSD1306Command::SetContrast)?;
        self.send_spi_command(0xCF)?;
        self.send_spi_command(SSD1306Command::SetPrecharge)?;
        self.send_spi_command(0xF1)?;
        self.send_spi_command(SSD1306Command::SetVComDetect)?;
        self.send_spi_command(0x40)?;
        self.send_spi_command(SSD1306Command::DisplayAllOnResume)?;
        self.send_spi_command(SSD1306Command::NormalDisplay)?;

        self.send_spi_command(SSD1306Command::DisplayOn)
    }

    fn reset(&mut self) -> error::Result<()> {
        self.rst_pin.set_value(1)?;
        thread::sleep(time::Duration::from_millis(1));
        self.rst_pin.set_value(0)?;
        thread::sleep(time::Duration::from_millis(10));
        self.rst_pin.set_value(1)?;
        Ok(())
    }

    fn flip_display(&mut self) {
        self.inverted = !self.inverted;
    }

    fn update(&mut self, bitmap: Bitmap) -> error::Result<()> {
        self.contents = Bitmap::new(self.contents.width(), self.contents.height());
        self.contents.blit(&bitmap, (0, 0));

        let width = self.contents.width() as u8;
        self.send_spi_command(SSD1306Command::ColumnAddr)?;
        self.send_spi_command(0)?;
        self.send_spi_command(width - 1)?;
        self.send_spi_command(SSD1306Command::PageAddr)?;
        self.send_spi_command(0)?;
        self.send_spi_command(width / 8 - 1)?;

        let pages = self.contents.height() / 8;
        let mut data = vec![];

        if self.inverted {
            for page in (0..pages).rev() {
                for x in (0..width).rev() {
                    let mut bits: u8 = 0;
                    for bit in 0..8 {
                        bits = bits << 1;
                        bits |= self.contents[page * 8 + bit][x as usize];
                    }
                    data.push(bits);
                }
            }
        } else {
            for page in 0..pages {
                for x in 0..width {
                    let mut bits: u8 = 0;
                    for bit in 0..8 {
                        bits = bits << 1;
                        bits |= self.contents[page * 8 + 7 - bit][x as usize];
                    }
                    data.push(bits);
                }
            }
        }
        self.send_spi_data(&data)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    pub struct TestDisplay {}

    impl Display for TestDisplay {
        fn on(&mut self) -> error::Result<()> {
            Ok(())
        }

        fn reset(&mut self) -> error::Result<()> {
            Ok(())
        }

        fn update(&mut self, _bitmap: Bitmap) -> error::Result<()> {
            Ok(())
        }

        fn flip_display(&mut self) {}

        fn width(&self) -> usize {
            0
        }

        fn height(&self) -> usize {
            0
        }
    }
}

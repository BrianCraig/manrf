mod app;

use esp_idf_sys as _;
use manrf::{defs::Runner, App}; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use std::time::Duration;
use std::{convert, thread};

use embedded_hal::spi::MODE_3;

use esp_idf_hal::delay::Ets;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::spi::*;
use esp_idf_hal::units::FromValueType;

use display_interface_spi::SPIInterfaceNoCS;

use embedded_graphics::pixelcolor::{Rgb565, Rgb888};
use embedded_graphics::prelude::*;
use embedded_graphics::{draw_target::ColorConverted, image::*};

use mipidsi::Builder;

fn main() {
    let peripherals = Peripherals::take().unwrap();
    println!("step1");
    let spi = peripherals.spi2; // Doesnt matter

    let sdo = peripherals.pins.gpio19; // MOSI master out slave in
    let sdi = peripherals.pins.gpio17; // Doesnt matter lol, doesn't communicate back

    let rst = PinDriver::output(peripherals.pins.gpio23).unwrap(); // ok
    let dc = PinDriver::output(peripherals.pins.gpio16).unwrap(); //ok
    let mut backlight = PinDriver::output(peripherals.pins.gpio4).unwrap(); //ok
    let sclk = peripherals.pins.gpio18; //ok
    let cs = peripherals.pins.gpio5; //ok

    let mut delay = Ets;

    // configuring the spi interface, note that in order for the ST7789 to work, the data_mode needs to be set to MODE_3
    let config = config::Config::new()
        .baudrate(26.MHz().into())
        .data_mode(MODE_3);

    println!("step2");
    let device =
        SpiDeviceDriver::new_single(spi, sclk, sdo, Some(sdi), Dma::Disabled, Some(cs), &config)
            .unwrap();

    // display interface abstraction from SPI and DC
    let di = SPIInterfaceNoCS::new(device, dc);

    // create driver
    let mut display = Builder::st7789_pico1(di)
        .init(&mut delay, Some(rst))
        .unwrap();

    // turn on the backlight
    backlight.set_high().unwrap();
    let raw_image_data = ImageRawLE::new(include_bytes!("ferris.raw"), 86);
    let ferris = Image::new(&raw_image_data, Point::new(0, 0));

    // draw image on black background
    display.clear(Rgb565::RED).unwrap();
    ferris.draw(&mut display).unwrap();

    type MyType<'a> = ColorConverted<'a,
        mipidsi::Display<
            SPIInterfaceNoCS<SpiDeviceDriver<'a,SpiDriver<'a>>, PinDriver<'a,Gpio16, Output>>,
            mipidsi::models::ST7789,
            PinDriver<'a,Gpio23, Output>,
        >,
        Rgb888,
    >;

    let converted_display: MyType = display.color_converted::<Rgb888>();
    
    let app = App::new(
        app::main_menu,
        embedded_graphics::geometry::Size::new(135, 240),
        converted_display,
    );

    loop {
        app.draw();
        thread::sleep(Duration::from_millis(100));
        if backlight.is_set_high() {
            backlight.set_low().unwrap();
        } else {
            backlight.set_high().unwrap();
        }
    }
}

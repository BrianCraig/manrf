use crate::Runner;

use super::{App, Box, Component, Number, Stack, Text};

use embedded_graphics::{geometry::Size, pixelcolor::Rgb565, prelude::RgbColor};

use embedded_graphics_simulator::SimulatorDisplay;

#[test]
fn create_leaf_app() {
    let comp: Component<()> = |()| Text::new("Hi");

    let numba: Component<()> = |()| Number::new(333);

    let mut display: SimulatorDisplay<embedded_graphics::pixelcolor::Rgb565> =
        SimulatorDisplay::new(Size::new(200, 400));

    let text_app = App::new(comp, (), &mut display);

    assert_eq!("Hi".to_string(), text_app.to_string());

    let numba_app = App::new(numba, (), &mut display);

    assert_eq!("333".to_string(), numba_app.to_string());
}

#[test]
fn create_multichild_app() {
    let comp: Component<()> = |()| Stack::col(vec![Text::new("Hi"), Number::new(333)]);

    let mut display: SimulatorDisplay<embedded_graphics::pixelcolor::Rgb565> =
        SimulatorDisplay::new(Size::new(200, 400));

    let app = App::new(comp, (), &mut display);

    assert_eq!("[Hi, 333]".to_string(), app.to_string());
}

#[test]
fn create_canvas() {
    let comp: Component<()> = |()| {
        let size_16 = Size {
            width: 16,
            height: 16,
        };
        Stack::col(vec![
            Box::exactly(size_16, Rgb565::RED, None),
            Box::exactly(size_16, Rgb565::GREEN, None),
            Box::exactly(size_16, Rgb565::BLUE, None),
        ])
    };

    let mut display: SimulatorDisplay<embedded_graphics::pixelcolor::Rgb565> =
        SimulatorDisplay::new(Size::new(200, 400));

    let app = App::new(comp, (), &mut display);

    app.render(Size::new(64, 64));
}

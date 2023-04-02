use std::any::TypeId;

use crate::example_components::ComponentDefinition;
use crate::{Padding, Runner};

use super::{App, Box, Component, Stack, Text};


use embedded_graphics::pixelcolor::raw::RawU16;
use embedded_graphics::prelude::{Point, RgbColor};
use embedded_graphics::{geometry::Size, pixelcolor::Rgb565};

use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, Window,
};

static palette: [u16; 16] = [
    0x0000_u16, 0x001F_u16, 0x03E0_u16, 0x7C00_u16, 0x7BE0_u16, 0xF800_u16, 0xFD20_u16, 0xFE00_u16,
    0xFFE0_u16, 0xFFFF_u16, 0x07E0_u16, 0x07FF_u16, 0x7C1F_u16, 0xF81F_u16, 0xFFE3_u16, 0xFFFF_u16,
];

#[test]
fn create_canvas() {
    let comp = ComponentDefinition::new(|_store| {
        let size_8 = Size {
            width: 8,
            height: 8,
        };
        Stack::col(vec![
            Box::exactly(size_8, Rgb565::RED, None),
            Box::exactly(size_8, Rgb565::GREEN, None),
            Box::exactly(size_8, Rgb565::BLUE, None),
        ])
    });

    let mut display: SimulatorDisplay<embedded_graphics::pixelcolor::Rgb565> =
        SimulatorDisplay::new(Size::new(30, 30));

    let mut app = App::new(comp);

    let output_settings = OutputSettingsBuilder::new()
        .max_fps(60)
        .pixel_spacing(1)
        .scale(8)
        .build();

    let _window = Window::new("Hello World", &output_settings);

    let a = app.render(Size::new(30, 30));

    app.paint(a, &mut display, Point::default());

    // window.show_static(&display);
}

#[test]
fn create_canvas_2() {
    let size = Size::new(80, 80);
    let comp = ComponentDefinition::new(|_store| {
        let size_8 = Size {
            width: 8,
            height: 8,
        };
        Stack::col(vec![
            Box::exactly(size_8, Rgb565::RED, None),
            Text::new("Hi"),
            Box::exactly(size_8, Rgb565::BLUE, None),
            Text::new("This is some Text"),
        ])
    });

    let mut display = SimulatorDisplay::new(size);

    let mut app = App::new(comp);

    let output_settings = OutputSettingsBuilder::new()
        .max_fps(60)
        .pixel_spacing(1)
        .scale(4)
        .build();

    let _window = Window::new("Hello World", &output_settings);

    let a = app.render(size);

    app.paint(a,  &mut display, Point::default());

    // window.show_static(&display);
}

#[test]
fn mutate_component() {
    let size = Size::new(80, 80);

    let comp= ComponentDefinition::new(|_store| {
        let size_8 = Size {
            width: 8,
            height: 8,
        };

        Stack::col(vec![
            Box::exactly(size_8, Rgb565::RED, None),
            Padding::new(Size::new(3, 2), Text::new("Hi")),
            Box::exactly(size_8, Rgb565::BLUE, None),
            Text::new("This is some Text"),
        ])
    });

    let mut display = SimulatorDisplay::new(size);

    let mut app = App::new(comp);

    let output_settings = OutputSettingsBuilder::new()
        .max_fps(60)
        .pixel_spacing(1)
        .scale(4)
        .build();

    let _window = Window::new("Hello World", &output_settings);

    let a = app.render(size);

    app.paint(a, &mut display, Point::default());

    // window.show_static(&display);
}

#[test]
fn create_colors() {
    let a: Rgb565 = Rgb565::from(RawU16::from(0xFEA0_u16));
    let a: Rgb565 = a;
    println!("{:?}", a);
    let a = TypeId::of::<Box>();
    println!("{:?}", a == TypeId::of::<Text>());
}

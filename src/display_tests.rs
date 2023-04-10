use crate::component::ComponentDefinition;
use crate::elements;
use crate::utils::EdgeInsets;
use crate::{Runner};

use super::{App, Box, Stack, Text};

use embedded_graphics::prelude::{Point, RgbColor};
use embedded_graphics::{geometry::Size, pixelcolor::Rgb565};

use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

#[test]
fn create_canvas() {
    let comp: ComponentDefinition<()> = ComponentDefinition::new(|_state| {
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

    let mut app: App<()> = App::new(comp);

    let output_settings = OutputSettingsBuilder::new()
        .max_fps(60)
        .pixel_spacing(1)
        .scale(8)
        .build();

    let _window = Window::new("Hello World", &output_settings);

    let a = app.render(Size::new(30, 30));

    app.paint(&a, &mut display, Point::default());

    // window.show_static(&display);
}

#[test]
fn create_canvas_2() {
    let size = Size::new(80, 80);
    let comp: ComponentDefinition<()> = ComponentDefinition::new(|_state| {
        let size_8 = Size {
            width: 8,
            height: 8,
        };
        Stack::col(vec![
            Box::exactly(size_8, Rgb565::RED, None),
            Text::new("Hi".to_string()),
            Box::exactly(size_8, Rgb565::BLUE, None),
            Text::new("This is some Text".to_string()),
        ])
    });

    let mut display = SimulatorDisplay::new(size);

    let mut app: App<()> = App::new(comp);

    let output_settings = OutputSettingsBuilder::new()
        .max_fps(60)
        .pixel_spacing(1)
        .scale(4)
        .build();

    let _window = Window::new("Hello World", &output_settings);

    let a = app.render(size);

    app.paint(&a, &mut display, Point::default());

    // window.show_static(&display);
}

#[test]
fn mutate_component() {
    let size = Size::new(80, 80);

    let comp: ComponentDefinition<()> = ComponentDefinition::new(|_state| {
        let size_8 = Size {
            width: 8,
            height: 8,
        };

        Stack::col(vec![
            Box::exactly(size_8, Rgb565::RED, None),
            elements::Padding::new(EdgeInsets::symmetric(2, 3), Text::new("Hi".to_string())),
            Box::exactly(size_8, Rgb565::BLUE, None),
            Text::new("This is some Text".to_string()),
        ])
    });

    let mut display = SimulatorDisplay::new(size);

    let mut app: App<()> = App::new(comp);

    let output_settings = OutputSettingsBuilder::new()
        .max_fps(60)
        .pixel_spacing(1)
        .scale(4)
        .build();

    let _window = Window::new("Hello World", &output_settings);

    let a = app.render(size);

    app.paint(&a, &mut display, Point::default());
}

use crate::{Runner, example_components::ComponentDefinition};

use super::{App, Box, Number, Stack, Text};

use embedded_graphics::{geometry::Size, pixelcolor::Rgb565, prelude::RgbColor};

#[test]
fn create_multichild_app() {
    let comp = ComponentDefinition::new(|_store| Stack::col(vec![Text::new("Hi"), Number::new(333)]));

    let mut app = App::new(comp);

    assert_eq!("[Hi, 333]".to_string(), app.to_string());
}

#[test]
fn create_canvas() {
    let comp = ComponentDefinition::new(|_| {
        let size_16 = Size {
            width: 16,
            height: 16,
        };
        Stack::col(vec![
            Box::exactly(size_16, Rgb565::RED, None),
            Box::exactly(size_16, Rgb565::GREEN, None),
            Box::exactly(size_16, Rgb565::BLUE, None),
        ])
    });

    let mut app = App::new(comp);

    app.render(Size::new(64, 64));
}

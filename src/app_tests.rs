use crate::{Runner, example_components::ComponentDefinition};

use super::{App, Box,  Stack};

use embedded_graphics::{geometry::Size, pixelcolor::Rgb565, prelude::RgbColor};

#[test]
fn create_canvas() {
    let comp:ComponentDefinition<()> = ComponentDefinition::new(|_| {
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

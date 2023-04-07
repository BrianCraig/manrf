use crate::defs::*;
use crate::utils::*;
use crate::component::ComponentDefinition;
use super::{App, Box,  Stack};

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

    let mut app: App<()> = App::new(comp);

    app.render(Size::new(64, 64));
}

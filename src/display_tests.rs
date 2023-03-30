use std::num::NonZeroU16;

use crate::Runner;

use super::{App, Component, Number, Stack, Text, Box};

use embedded_graphics::pixelcolor::raw::RawU16;
use embedded_graphics::{geometry::Size, pixelcolor::Rgb565};
use embedded_graphics::mock_display::MockDisplay;

use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,SimulatorEvent
};


static palette: [u16; 16] = [
    0x0000_u16,
    0x001F_u16,
    0x03E0_u16,
    0x7C00_u16,
    0x7BE0_u16,
    0xF800_u16,
    0xFD20_u16,
    0xFE00_u16,
    0xFFE0_u16,
    0xFFFF_u16,
    0x07E0_u16,
    0x07FF_u16,
    0x7C1F_u16,
    0xF81F_u16,
    0xFFE3_u16,
    0xFFFF_u16,
];

#[test]
fn create_canvas() {
    let comp: Component<()> = |()| {
        let size_100 = Size{ width: 100, height:100 };
        Stack::col(vec![
            Box::exactly(size_100, None),
            Box::exactly(size_100, None),
            Box::exactly(size_100, None),
        ])
    };

    let mut display: SimulatorDisplay<embedded_graphics::pixelcolor::Rgb565> = SimulatorDisplay::new(Size::new(200, 400));

    let mut app = App::new(comp, (), &mut display);

    let output_settings = OutputSettingsBuilder::new()
        .max_fps(60)
        .pixel_spacing(1)
        .scale(1)
        .build();

    let mut window = Window::new("Hello World", &output_settings);

    app.render(Size::new(200, 400));

    window.show_static(&display);
    
    // how it should be called
    
    // app.child().determine_size(constraints) -> Size
    
    // col.for child
    //   child.determine_size(constraints - sum_size) -> Size
    
    //   return exactly
    
    // return Size child.size.height.sum, child.size.width.max
    
    // app center col
    
    // each element should understand paint(pos, &display), determine_size(constraints) -> Size, and know its size.
    
    // it also has to know each child position.
    
}


#[test]
fn create_colors() {
    let a: Rgb565 = Rgb565::from(RawU16::from(0xFEA0_u16));
    let a: Rgb565 = a.into();
    println!("{:?}", a);

}
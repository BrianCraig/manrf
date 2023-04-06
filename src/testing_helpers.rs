use embedded_graphics::prelude::{Point, Size};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

use crate::{event::Event, example_components::ComponentDefinition, App, Runner};

type TINR<T> = fn(Size, &mut dyn Runner<T>);

pub fn test_in_window<T: Default>(size: Size, comp: ComponentDefinition<T>, callback: TINR<T>) {
    let mut display = SimulatorDisplay::new(size);

    let mut app = App::new(comp);

    let output_settings = OutputSettingsBuilder::new()
        .max_fps(60)
        .pixel_spacing(1)
        .scale(4)
        .build();

    let mut window = Window::new("Hello World", &output_settings);

    'running: loop {
        let nodes = app.render(size);
        app.paint(nodes, &mut display, Point::default());
        window.update(&display);
        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                other => {
                    let event = Event::try_from(other);
                    if let Ok(event) = event {
                        app.handle_event(event);
                        callback(size, &mut app);
                    }
                }
            }
        }
    }
}

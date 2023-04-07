use std::time::SystemTime;

use crate::utils::*;
use crate::defs::*;
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

use crate::{ component::ComponentDefinition, App};

type TINR<T> = fn(Size, &mut dyn Runner<T>);

#[allow(dead_code)]
pub fn test_in_window<T: Default>(size: Size, comp: ComponentDefinition<T>, callback: TINR<T>) {
    let mut display = SimulatorDisplay::new(size);

    let mut app = App::new(comp);

    let mut frames_counter = (SystemTime::now(), 0);

    let output_settings = OutputSettingsBuilder::new()
        .max_fps(60)
        .pixel_spacing(1)
        .scale(4)
        .build();

    let mut window = Window::new("Hello World", &output_settings);

    'running: loop {
        let nodes = app.render(size);
        app.paint(&nodes, &mut display, Point::default());
        window.update(&display);

        frames_counter.1 += 1;
        if frames_counter.0.elapsed().unwrap().as_secs() >= 1 {
            println!("FPS: {}", frames_counter.1);
            frames_counter = (SystemTime::now(), 0);
        }
        
        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                other => {
                    let event = Event::try_from(other);
                    if let Ok(event) = event {
                        app.handle_event(event, &nodes);
                        callback(size, &mut app);
                    }
                }
            }
        }
    }
}

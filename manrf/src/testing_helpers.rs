use std::time::SystemTime;

use crate::defs::*;
use crate::utils::*;
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

use crate::{App, ComponentGenerator};

type TINR = fn(Size, &mut dyn Runner);

#[allow(dead_code)]
pub fn test_in_window<S: Default>(size: Size, comp: ComponentGenerator<S, SimulatorDisplay<Rgb888>>, callback: TINR) {
    let display:SimulatorDisplay<Rgb888> = SimulatorDisplay::new(size);

    let mut app = App::new(comp, size, display);

    let mut frames_counter = (SystemTime::now(), 0);

    let output_settings = OutputSettingsBuilder::new()
        .max_fps(60)
        .pixel_spacing(0)
        .scale(4)
        .build();

    let mut window = Window::new("Hello World", &output_settings);

    'running: loop {
        app.draw();
        window.update(&app.target);

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
                        app.handle_event(event);
                        callback(size, &mut app);
                    }
                }
            }
        }
    }
}

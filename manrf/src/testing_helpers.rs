use std::time::SystemTime;

use crate::defs::*;
use crate::utils::*;
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

use crate::{App, ComponentGenerator};

type TINR<T> = fn(Size, &mut dyn Runner<T>);

#[allow(dead_code)]
pub fn test_in_window<T: Default + 'static>(size: Size, comp: ComponentGenerator<T>, callback: TINR<T>) {
    let mut display = SimulatorDisplay::new(size);

    let mut app = App::new(comp, size);

    let mut frames_counter = (SystemTime::now(), 0);

    let output_settings = OutputSettingsBuilder::new()
        .max_fps(60)
        .pixel_spacing(0)
        .scale(4)
        .build();

    let mut window = Window::new("Hello World", &output_settings);

    'running: loop {
        app.draw(&mut display);
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
                        app.handle_event(event);
                        callback(size, &mut app);
                    }
                }
            }
        }
    }
}

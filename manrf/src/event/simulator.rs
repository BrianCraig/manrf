use embedded_graphics_simulator::{
    sdl2::{Keycode, MouseButton},
    SimulatorEvent,
};

use super::{Event, Button, Direction};

impl TryFrom<SimulatorEvent> for Event {
    type Error = ();

    fn try_from(event: SimulatorEvent) -> Result<Self, Self::Error> {
        match event {
            SimulatorEvent::MouseButtonDown { mouse_btn, .. } => match mouse_btn {
                MouseButton::Left => Ok(Event::ButtonPressed(Button::Principal)),
                MouseButton::Right => Ok(Event::ButtonPressed(Button::Back)),
                _ => Err(()),
            },
            SimulatorEvent::KeyDown { keycode, .. } => match keycode {
                Keycode::Up => Ok(Event::DirectionPressed(Direction::Up)),
                Keycode::Down => Ok(Event::DirectionPressed(Direction::Down)),
                Keycode::Left => Ok(Event::DirectionPressed(Direction::Left)),
                Keycode::Right => Ok(Event::DirectionPressed(Direction::Right)),
                Keycode::Z => Ok(Event::ButtonPressed(Button::Principal)),
                Keycode::X => Ok(Event::ButtonPressed(Button::Back)),
                Keycode::Space => Ok(Event::ButtonPressed(Button::Secondary)),
                _ => Err(()),
            },
            _ => Err(()),
        }
    }
}

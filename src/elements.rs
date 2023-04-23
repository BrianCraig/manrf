use embedded_graphics::pixelcolor::Rgb888;
use crate::{defs::Element, utils::EdgeInsets};

mod component;
mod handler;
mod style;

pub use component::*;
pub use handler::*;
pub use style::*;


pub fn border<S: 'static>(border: BorderDefinition, child: Element<S>) -> Element<S> {
    Style::new(None, EdgeInsets::all(0), border, EdgeInsets::all(0), child)
}

pub fn padding<S: 'static>(padding: EdgeInsets, child: Element<S>) -> Element<S> {
    Style::new(
        None,
        EdgeInsets::all(0),
        BorderDefinition::none(),
        padding,
        child,
    )
}

pub fn background<S: 'static>(background: Rgb888, child: Element<S>) -> Element<S> {
    Style::new(
        Some(background),
        EdgeInsets::all(0),
        BorderDefinition::none(),
        EdgeInsets::all(0),
        child,
    )
}

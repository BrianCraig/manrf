use embedded_graphics::pixelcolor::Rgb888;
use crate::{defs::{Element, State, Target888}, utils::EdgeInsets};

mod component;
mod handler;
mod style;
mod align;

pub use component::*;
pub use handler::*;
pub use style::*;
pub use align::*;


pub fn border<S: State, T:Target888>(border: BorderDefinition, child: Element<S, T>) -> Element<S, T> {
    Style::new(None, EdgeInsets::all(0), border, EdgeInsets::all(0), child)
}

pub fn padding<S: State, T:Target888>(padding: EdgeInsets, child: Element<S, T>) -> Element<S, T> {
    Style::new(
        None,
        EdgeInsets::all(0),
        BorderDefinition::none(),
        padding,
        child,
    )
}

pub fn background<S: State, T:Target888>(background: Rgb888, child: Element<S, T>) -> Element<S, T> {
    Style::new(
        Some(background),
        EdgeInsets::all(0),
        BorderDefinition::none(),
        EdgeInsets::all(0),
        child,
    )
}

pub fn center<S: State, T:Target888>(child: Element<S, T>) -> Element<S, T> {
    Align::new(
        Alignment2D {
            horizontal: Alignment::Center,
            vertical: Alignment::Center,
        },
        child,
    )
}

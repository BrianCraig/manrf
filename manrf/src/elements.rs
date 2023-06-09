use crate::{
    defs::{Element, State},
    utils::EdgeInsets,
};
use embedded_graphics::{pixelcolor::Rgb888};

mod align;
mod component;
mod handler;
mod style;

pub use align::*;
pub use component::*;
pub use handler::*;
pub use style::*;

pub fn border<S: State>(border: BorderDefinition, child: Element<S>) -> Element<S> {
    Style::new(None, EdgeInsets::all(0), border, EdgeInsets::all(0), child)
}

pub fn padding<S: State>(padding: EdgeInsets, child: Element<S>) -> Element<S> {
    Style::new(
        None,
        EdgeInsets::all(0),
        BorderDefinition::none(),
        padding,
        child,
    )
}

pub fn background<S: State>(background: Rgb888, child: Element<S>) -> Element<S> {
    Style::new(
        Some(background),
        EdgeInsets::all(0),
        BorderDefinition::none(),
        EdgeInsets::all(0),
        child,
    )
}

pub fn center<S: State>(child: Element<S>) -> Element<S> {
    Align::new(
        Alignment2D {
            horizontal: Alignment::Center,
            vertical: Alignment::Center,
        },
        child,
    )
}

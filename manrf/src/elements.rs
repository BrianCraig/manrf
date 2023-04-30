use crate::{
    defs::{Element, State},
    utils::EdgeInsets,
};
use embedded_graphics::{pixelcolor::Rgb888, prelude::DrawTarget};

mod align;
mod component;
mod handler;
mod style;

pub use align::*;
pub use component::*;
pub use handler::*;
pub use style::*;

pub fn border<
    S: State,
    T: DrawTarget<Color = Rgb888> + 'static,
>(
    border: BorderDefinition,
    child: Element<S, T>,
) -> Element<S, T> {
    Style::new(None, EdgeInsets::all(0), border, EdgeInsets::all(0), child)
}

pub fn padding<
    S: State,
    T: DrawTarget<Color = Rgb888> + 'static,
>(
    padding: EdgeInsets,
    child: Element<S, T>,
) -> Element<S, T> {
    Style::new(
        None,
        EdgeInsets::all(0),
        BorderDefinition::none(),
        padding,
        child,
    )
}

pub fn background<
    S: State,
    T: DrawTarget<Color = Rgb888> + 'static,
>(
    background: Rgb888,
    child: Element<S, T>,
) -> Element<S, T> {
    Style::new(
        Some(background),
        EdgeInsets::all(0),
        BorderDefinition::none(),
        EdgeInsets::all(0),
        child,
    )
}

pub fn center<
    S: State,
    T: DrawTarget<Color = Rgb888> + 'static,
>(
    child: Element<S, T>,
) -> Element<S, T> {
    Align::new(
        Alignment2D {
            horizontal: Alignment::Center,
            vertical: Alignment::Center,
        },
        child,
    )
}

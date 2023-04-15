mod style;

pub use style::*;

use crate::{defs::Element, utils::EdgeInsets};

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

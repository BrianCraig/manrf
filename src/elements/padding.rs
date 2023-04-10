use crate::{defs::Element, utils::EdgeInsets};

use super::{BorderDefinition, Style};

pub struct Padding {}

impl Padding {
    pub fn new<S: 'static>(padding: EdgeInsets, child: Element<S>) -> Element<S> {
        Style::new(
            None,
            EdgeInsets::all(0),
            BorderDefinition::none(),
            padding,
            Some(child),
        )
    }
}

use crate::{defs::Element, utils::EdgeInsets};

use super::{BorderDefinition, Style};

pub struct Border {}

impl Border {
    pub fn new<S: 'static>(border: BorderDefinition, child: Element<S>) -> Element<S> {
        Style::new(
            None,
            EdgeInsets::all(0),
            border,
            EdgeInsets::all(0),
            Some(child),
        )
    }
}

use embedded_graphics::prelude::{Point, Size};

use crate::Element;

#[derive(Clone)]
pub enum RenderNode {
    SingleChild {
        offset: Point,
        size: Size,
        renderer: Element,
        child: Box<RenderNode>,
    },
    MultiChild {
        offset: Point,
        size: Size,
        child: Vec<RenderNode>,
    },
    Leaf,
}

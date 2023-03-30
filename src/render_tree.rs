use embedded_graphics::prelude::{Point, Size};

use crate::Element;

#[derive(Clone)]
pub struct RenderData {
    pub offset: Point,
    pub size: Size,
    pub renderer: Element,
    pub child: Box<RenderNode>,
}

#[derive(Clone)]
pub enum RenderNode {
    SingleChild(RenderData),
    MultiChild {
        offset: Point,
        size: Size,
        child: Vec<RenderNode>,
    },
    Leaf,
}

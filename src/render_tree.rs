use embedded_graphics::prelude::{Point, Size};

use crate::{Element};
struct RenderData{
    offset: Point,
    size: Size,
    renderer: Element,
    child: Box<RenderNode>
}

enum RenderNode {
    SingleChild(RenderData),
    MultiChild(Vec<RenderData>),
    Leaf
}


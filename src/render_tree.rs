use embedded_graphics::prelude::{Point, Size};

use crate::Element;

#[derive(Clone)]
pub enum RenderNode<T> {
    SingleChild {
        offset: Point,
        size: Size,
        renderer: Element<T>,
        child: Box<RenderNode<T>>,
    },
    MultiChild {
        offset: Point,
        size: Size,
        child: Vec<RenderNode<T>>,
    },
    Leaf,
}

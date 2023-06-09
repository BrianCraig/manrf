use crate::defs::*;
use crate::graphics::{GraphicOperation, GraphicOperationQueue};
use crate::utils::*;

pub struct BorderDefinition {
    pub color: Rgb888,
    pub size: EdgeInsets,
}

impl BorderDefinition {
    pub const fn new(color: Rgb888, size: EdgeInsets) -> Self {
        Self { color, size }
    }

    pub const fn none() -> Self {
        Self {
            color: Rgb888::BLACK,
            size: EdgeInsets::all(0),
        }
    }
}

pub struct StyleDefinition {
    pub background: Option<Rgb888>,
    pub margin: EdgeInsets,
    pub border: BorderDefinition,
    pub padding: EdgeInsets,
}

pub struct Style<S> {
    style: StyleDefinition,
    child: Element<S>,
}

impl<S> Style<S> {
    pub fn new(
        background: Option<Rgb888>,
        margin: EdgeInsets,
        border: BorderDefinition,
        padding: EdgeInsets,
        child: Element<S>,
    ) -> Rc<Self> {
        Rc::new(Self {
            style: StyleDefinition {
                background,
                margin,
                border,
                padding,
            },
            child,
        })
    }

    pub fn new_with_style(style: StyleDefinition, child: Element<S>) -> Rc<Self> {
        Rc::new(Self { style, child })
    }

    pub fn new_with_background(background: Rgb888, child: Element<S>) -> Rc<Self> {
        Rc::new(Self {
            style: StyleDefinition {
                background: Some(background),
                margin: EdgeInsets::all(0),
                border: BorderDefinition::none(),
                padding: EdgeInsets::all(0),
            },
            child,
        })
    }

    fn added_size(&self) -> Size {
        self.style.margin.size() + self.style.border.size.size() + self.style.padding.size()
    }

    fn child_offset(&self) -> Point {
        self.style.margin.top_left_offset()
            + self.style.border.size.top_left_offset()
            + self.style.padding.top_left_offset()
    }

    fn background_offset(&self) -> Point {
        self.style.margin.top_left_offset() + self.style.border.size.top_left_offset()
    }

    fn background_size(&self, size: Size) -> Size {
        size - self.style.margin.size() - self.style.border.size.size()
    }

    fn border_offset(&self) -> Point {
        self.style.margin.top_left_offset()
    }

    fn border_size(&self, size: Size) -> Size {
        size - self.style.margin.size()
    }
}

impl<S: State> ElementTrait<S> for Style<S> {
    fn render(&self, _constraints: Constraints, state: &S) -> (Size, RenderNode<S>) {
        let added_size = self.added_size();
        let constraints = _constraints.shrink(&added_size);

        let (size, render_node) = self.child.render(constraints, state);
        (
            size + added_size,
            RenderNode::SingleChild {
                offset: self.child_offset(),
                child: std::boxed::Box::new(render_node),
                renderer: self.child.clone(),
                size,
            },
        )
    }

    #[allow(unused_must_use)]
    fn paint(&self, size: Size, pos: Point, queue: &mut GraphicOperationQueue) {
        if let Some(color) = self.style.background {
            queue.push(GraphicOperation::DrawRectangle {
                rect: Rectangle::new(self.background_offset() + pos, self.background_size(size)),
                color: color,
            });
        }
        if !self.style.border.size.is_empty() {
            // paint left border
            queue.push(GraphicOperation::DrawRectangle {
                rect: Rectangle::new(
                    self.border_offset() + pos,
                    Size::new(self.style.border.size.left, self.border_size(size).height),
                ),
                color: self.style.border.color,
            });

            // paint right border
            queue.push(GraphicOperation::DrawRectangle {
                rect: Rectangle::new(
                    self.border_offset()
                        + pos
                        + Point::new(
                            self.border_size(size).width as i32
                                - self.style.border.size.right as i32,
                            0,
                        ),
                    Size::new(self.style.border.size.right, self.border_size(size).height),
                ),
                color: self.style.border.color,
            });

            // paint top border
            queue.push(GraphicOperation::DrawRectangle {
                rect: Rectangle::new(
                    self.border_offset() + pos,
                    Size::new(self.border_size(size).width, self.style.border.size.top),
                ),
                color: self.style.border.color,
            });

            // paint bottom border
            queue.push(GraphicOperation::DrawRectangle {
                rect: Rectangle::new(
                    self.border_offset()
                        + pos
                        + Point::new(
                            0,
                            self.border_size(size).height as i32
                                - self.style.border.size.bottom as i32,
                        ),
                    Size::new(self.border_size(size).width, self.style.border.size.bottom),
                ),
                color: self.style.border.color,
            });
        }
    }
}

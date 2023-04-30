use crate::defs::*;
use crate::utils::*;

pub enum Alignment {
    Start,
    Center,
    End,
}

pub struct Alignment2D {
    pub horizontal: Alignment,
    pub vertical: Alignment,
}

pub struct Align<'a, S: Default, T> {
    alignment: Alignment2D,
    child: Element<'a, S, T>,
}

impl<'a, S: Default, T> Align<'a, S, T> {
    pub fn new(alignment: Alignment2D, child: Element<'a, S, T>) -> Rc<Self> {
        Rc::new(Self { alignment, child })
    }
}

impl<'a, S: Default, T: DrawTarget<Color = Rgb888>> ElementTrait<'a, S, T> for Align<'a, S, T> {
    fn render(&self, constraints: Constraints, state: &'a S) -> (Size, RenderNode<'a, S, T>) {
        let (child_size, child_node) = self.child.render(constraints, state);
        let size = constraints.max;
        let offset_x = match self.alignment.horizontal {
            Alignment::Start => 0,
            Alignment::Center => (size.width - child_size.width) / 2,
            Alignment::End => size.width - child_size.width,
        };
        let offset_y = match self.alignment.vertical {
            Alignment::Start => 0,
            Alignment::Center => (size.height - child_size.height) / 2,
            Alignment::End => size.height - child_size.height,
        };
        (
            size,
            RenderNode::SingleChild {
                offset: Point::new(offset_x as i32, offset_y as i32),
                size: child_size,
                renderer: self.child.clone(),
                child: std::boxed::Box::new(child_node),
            },
        )
    }
}

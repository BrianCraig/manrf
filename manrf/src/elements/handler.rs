use crate::defs::*;
use crate::utils::*;

pub type EventHandler< S> = fn(&mut S, Event) -> bool;

pub struct Handler<'a, S, T> {
    handler: EventHandler< S>,
    child: Element<'a, S, T>,
}

impl<'a, S: Default, T> Handler<'a, S, T> {
    pub fn new(handler: EventHandler< S>, child: Element<'a, S, T>) -> Rc<Self> {
        Rc::new(Self { handler, child })
    }
}

impl<'a, S: Default, T: DrawTarget<Color = Rgb888>> ElementTrait<'a, S, T> for Handler<'a, S, T> {
    fn render(&self, constraints: Constraints, state: &'a S) -> (Size, RenderNode<'a, S, T>) {
        let (size, child_node) = self.child.render(constraints, state);
        (
            size,
            RenderNode::SingleChild {
                offset: Point::default(),
                size,
                renderer: self.child.clone(),
                child: Box::new(child_node),
            },
        )
    }

    fn event_handler(&self, state: &'a mut S, event: Event) -> bool {
        (self.handler)(state, event)
    }
}

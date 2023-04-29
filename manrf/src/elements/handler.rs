use crate::defs::*;
use crate::utils::*;

pub type EventHandler<S> = fn(&mut S, Event) -> bool;

pub struct Handler<S, T> {
    handler: EventHandler<S>,
    child: Element<S, T>,
}

impl<S: State, T:Target888> Handler<S, T> {
    pub fn new(handler: EventHandler<S>, child: Element<S, T>) -> Rc<Self> {
        Rc::new(Self { handler, child })
    }
}

impl<S: State, T:Target888> ElementTrait<S, T> for Handler<S, T> {
    fn render(&self, constraints: Constraints, state: &S) -> (Size, RenderNode<S, T>) {
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

    fn event_handler(&self, state: &mut S, event: Event) -> bool {
        (self.handler)(state, event)
    }
}

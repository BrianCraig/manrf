use crate::defs::*;
use crate::utils::*;

pub type EventHandler<S> = fn(&mut S, Event) -> bool;

pub struct Handler<S> {
    handler: EventHandler<S>,
    child: Element<S>,
}

impl<S: State> Handler<S> {
    pub fn new(handler: EventHandler<S>, child: Element<S>) -> Rc<Self> {
        Rc::new(Self { handler, child })
    }
}

impl<S: State> ElementTrait<S> for Handler<S> {
    fn render(&self, constraints: Constraints, state: &S) -> (Size, RenderNode<S>) {
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

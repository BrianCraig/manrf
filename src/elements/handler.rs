use crate::defs::*;
use crate::utils::*;

pub struct Handler<S> {
    child: Element<S>,
    handler: fn(&mut S, Event) -> bool,
}

impl<S> Handler<S> {
    pub fn new(child: Element<S>, handler: fn(&mut S, Event) -> bool) -> Rc<Self> {
        Rc::new(Self { child, handler })
    }
}

impl<S> ElementTrait<S> for Handler<S> {
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

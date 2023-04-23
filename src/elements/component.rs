use crate::defs::*;
use crate::utils::*;

pub struct Component<S> {
    generator: fn(&S) -> Element<S>,
}

impl<S> Component<S> {
    pub fn new(generator: fn(&S) -> Element<S>) -> Rc<Self> {
        Rc::new(Self { generator })
    }
}

impl<S> ElementTrait<S> for Component<S> {
    fn render(&self, constraints: Constraints, state: &S) -> (Size, RenderNode<S>) {
        let child: Rc<dyn ElementTrait<S>> = (self.generator)(state);
        let (size, child_node) = child.render(constraints, state);
        (
            size,
            RenderNode::SingleChild {
                offset: Point::default(),
                size,
                renderer: child.clone(),
                child: Box::new(child_node),
            },
        )
    }

    fn event_handler(&self, state: &mut S, event: Event) -> bool {
        (self.generator)(state).event_handler(state, event)
    }
}

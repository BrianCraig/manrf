use crate::defs::*;
use crate::utils::*;

pub type Generator<S> = fn(&S) -> Element<S>;

pub struct Component<S> {
    generator: Generator<S>,
}

impl<S: State> Component<S> {
    pub fn new(generator: Generator<S>) -> Rc<Self> {
        Rc::new(Self { generator })
    }
}

impl<S: State> ElementTrait<S> for Component<S> {
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

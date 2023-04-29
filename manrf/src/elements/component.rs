use crate::defs::*;
use crate::utils::*;

pub type Generator<S, T> = fn(&S) -> Element<S, T>;

pub struct Component<S, T> {
    generator: Generator<S, T>,
}

impl<S: State, T:Target888> Component<S, T> {
    pub fn new(generator: Generator<S, T>) -> Rc<Self> {
        Rc::new(Self { generator })
    }
}

impl<S: State, T:Target888> ElementTrait<S, T> for Component<S, T> {
    fn render(&self, constraints: Constraints, state: &S) -> (Size, RenderNode<S, T>) {
        let child: Rc<dyn ElementTrait<S, T>> = (self.generator)(state);
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

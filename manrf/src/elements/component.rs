use crate::defs::*;
use crate::utils::*;

pub type Generator<S, T> = fn(&S) -> Element<S, T>;

pub struct Component<S, T> {
    generator: Generator<S, T>,
}

impl<'a, S: Default, T> Component<S, T> {
    pub fn new(generator: Generator<S, T>) -> Rc<Self> {
        Rc::new(Self { generator })
    }
}

impl<'a, S: Default, T: DrawTarget<Color = Rgb888>> ElementTrait<'a, S, T> for Component<S, T> {
    fn render(&self, constraints: Constraints, state: &'a S) -> (Size, RenderNode<'a, S, T>) {
        let child: Rc<dyn ElementTrait<'a, S, T>> = (self.generator)(state);
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

    fn event_handler(&self, state: &'a mut S, event: Event) -> bool {
        let b:&S =  &*state;
        let elem: Element<S, T> = (self.generator)(b);
        drop(b);
        let res = elem.event_handler(state, event);
        res
    }
}

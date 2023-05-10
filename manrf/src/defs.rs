use crate::{utils::*, graphics::GraphicOperationQueue};

pub trait State: Default + 'static {}

pub trait ElementTrait<S: State>{
    fn to_string(&self) -> String {
        todo!()
    }
    fn render(&self, constraints: Constraints, state: &S) -> (Size, RenderNode<S>);
    fn paint(&self, _size: Size, _pos: Point, _display: &mut GraphicOperationQueue) {}
    fn event_handler(&self, _state: &mut S, _event: Event) -> bool {
        false
    }
}

pub type Element<S> = Rc<dyn ElementTrait<S>>;

pub type EventFunction<S> = fn(&mut S, Event) -> bool;

pub type ComponentGenerator<S> = fn(&S) -> Element<S>;

#[derive(Clone)]
pub enum RenderNode<S> {
    SingleChild {
        offset: Point,
        size: Size,
        renderer: Element<S>,
        child: Box<RenderNode<S>>,
    },
    MultiChild {
        offset: Point,
        size: Size,
        child: Vec<RenderNode<S>>,
    },
    Leaf,
}
pub trait Runner {
    #[deprecated]
    fn to_string(&mut self) -> String;
    fn handle_event(&mut self, event: crate::event::Event);
    fn draw(&mut self);
}
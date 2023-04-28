use embedded_graphics_simulator::SimulatorDisplay;
use crate::utils::*;

pub trait State: Default + 'static {}

pub type Draw565 = SimulatorDisplay<Rgb565>;
pub trait ElementTrait<S: State> {
    fn to_string(&self) -> String {
        todo!()
    }
    fn render(&self, constraints: Constraints, state: &S) -> (Size, RenderNode<S>);
    fn paint(&self, _size: Size, _pos: Point, _display: &mut Draw565) {}
    fn event_handler(&self, _state: &mut S, _event: Event) -> bool {
        false
    }
}

pub type Element<S> = Rc<dyn ElementTrait<S>>;

pub type EventFunction<T> = fn(&mut T, Event) -> bool;

pub type ComponentGenerator<S> = fn(&S) -> Element<S>;

#[derive(Clone)]
pub enum RenderNode<T> {
    SingleChild {
        offset: Point,
        size: Size,
        renderer: Element<T>,
        child: Box<RenderNode<T>>,
    },
    MultiChild {
        offset: Point,
        size: Size,
        child: Vec<RenderNode<T>>,
    },
    Leaf,
}
pub trait Runner {
    #[deprecated]
    fn to_string(&mut self) -> String;
    fn handle_event(&mut self, event: crate::event::Event);
    fn draw(&mut self, target: &mut Draw565);
}
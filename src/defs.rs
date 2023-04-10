use embedded_graphics_simulator::SimulatorDisplay;
use crate::utils::*;

pub type Draw565 = SimulatorDisplay<Rgb565>;
pub trait ElementTrait<S> {
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

pub type ComponentGenerator<T> = fn(&mut T) -> Element<T>;

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
pub trait Runner<S> {
    fn to_string(&mut self) -> String;
    fn render(&mut self, size: Size) -> RenderNode<S>;
    fn paint(&mut self, node: &RenderNode<S>, target: &mut Draw565, offset: Point);
}
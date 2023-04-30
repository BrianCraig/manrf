use embedded_graphics_simulator::SimulatorDisplay;
use crate::utils::*;

pub trait State: Default + 'static {}

pub type Draw565 = SimulatorDisplay<Rgb565>;

pub trait ElementTrait<S: State, T:DrawTarget<Color = Rgb888>>{
    fn to_string(&self) -> String {
        todo!()
    }
    fn render(&self, constraints: Constraints, state: &S) -> (Size, RenderNode<S, T>);
    fn paint(&self, _size: Size, _pos: Point, _display: &mut T) {}
    fn event_handler(&self, _state: &mut S, _event: Event) -> bool {
        false
    }
}

pub type Element<S, T> = Rc<dyn ElementTrait<S, T>>;

pub type EventFunction<T> = fn(&mut T, Event) -> bool;

pub type ComponentGenerator<S, T> = fn(&S) -> Element<S, T>;

#[derive(Clone)]
pub enum RenderNode<S, T> {
    SingleChild {
        offset: Point,
        size: Size,
        renderer: Element<S, T>,
        child: Box<RenderNode<S, T>>,
    },
    MultiChild {
        offset: Point,
        size: Size,
        child: Vec<RenderNode<S, T>>,
    },
    Leaf,
}
pub trait Runner {
    #[deprecated]
    fn to_string(&mut self) -> String;
    fn handle_event(&mut self, event: crate::event::Event);
    fn draw(&mut self);
}
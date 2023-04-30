use crate::utils::*;
use embedded_graphics_simulator::SimulatorDisplay;

pub type Draw565 = SimulatorDisplay<Rgb565>;

pub trait ElementTrait<'a, S: Default, T: DrawTarget<Color = Rgb888>> {
    fn to_string(&self) -> String {
        todo!()
    }
    fn render(&self, constraints: Constraints, state: &'a S) -> (Size, RenderNode<'a, S, T>);
    fn paint(&self, _size: Size, _pos: Point, _display: &mut T) {}
    fn event_handler(&self, _state: &'a mut S, _event: Event) -> bool {
        false
    }
}

pub type Element<'a, S, T> = Rc<dyn ElementTrait<'a, S, T>>;

pub type EventFunction<T> = fn(&mut T, Event) -> bool;

pub type ComponentGenerator<S, T> = fn(&S) -> Element<S, T>;

#[derive(Clone)]
pub enum RenderNode<'a, S, T> {
    SingleChild {
        offset: Point,
        size: Size,
        renderer: Element<'a, S, T>,
        child: Box<RenderNode<'a, S, T>>,
    },
    MultiChild {
        offset: Point,
        size: Size,
        child: Vec<RenderNode<'a, S, T>>,
    },
    Leaf,
}
pub trait Runner {
    #[deprecated]
    fn to_string(&mut self) -> String;
    fn handle_event(&mut self, event: crate::event::Event);
    fn draw(&mut self);
}

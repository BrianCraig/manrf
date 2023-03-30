#![feature(fn_traits)]

use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::Size;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::{Point, RgbColor};
use embedded_graphics::primitives::Rectangle;
use embedded_graphics_simulator::SimulatorDisplay;
use std::rc::Rc;

mod constraints;
mod render_tree;

fn main() {}

pub trait Leaf {
    fn to_string(&self) -> String;
    fn determine_size(&mut self, constraints: constraints::Constraints) -> Size;
    fn paint(&self, pos: Point, display: &mut Draw565);
}

type Element = Rc<dyn Leaf>;

pub struct Stack {
    items: Vec<Element>,
}

impl Stack {
    pub fn col(items: Vec<Element>) -> Rc<Self> {
        Rc::new(Stack { items })
    }
}

impl Leaf for Stack {
    fn to_string(&self) -> String {
        let coll = self
            .items
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        format!("[{}]", coll)
    }

    fn determine_size(&mut self, constraints: constraints::Constraints) -> Size {
        Size {
            width: 10,
            height: 10
        }
    }

    fn paint(&self, pos: Point, display: &mut Draw565){
        todo!()
    }
}

pub struct Box {
    size: Size,
}

impl Box {
    pub fn exactly(size: Size, _: Option<i32>) -> Rc<Self> {
        Rc::new(Self { size })
    }
}

impl Leaf for Box {
    fn to_string(&self) -> String {
        todo!()
    }

    fn determine_size(&mut self, constraints: constraints::Constraints) -> Size {
        todo!()
    }

    fn paint(&self, pos: Point, display: &mut Draw565){
        todo!()
    }
}

pub struct Text {
    val: &'static str,
}

impl Text {
    pub fn new(val: &'static str) -> Rc<Self> {
        Rc::new(Self { val })
    }
}

impl Leaf for Text {
    fn to_string(&self) -> String {
        self.val.to_string()
    }

    fn determine_size(&mut self, constraints: constraints::Constraints) -> Size {
        todo!()
    }

    fn paint(&self, pos: Point, display: &mut Draw565){
        todo!()
    }
}

pub struct Number {
    val: i32,
}

impl Number {
    pub fn new(val: i32) -> Rc<Self> {
        Rc::new(Self { val })
    }
}

impl Leaf for Number {
    fn to_string(&self) -> String {
        self.val.to_string()
    }

    fn determine_size(&mut self, constraints: constraints::Constraints) -> Size {
        todo!()
    }

    fn paint(&self, pos: Point, display: &mut Draw565){
        todo!()
    }
}

type Component<T> = fn(T) -> Element;

type Draw565 = SimulatorDisplay<Rgb565>;

trait Runner {
    fn to_string(&self) -> String;
    fn render(&mut self, size: Size);
}

pub struct App<'a, T>
where
    T: Copy,
{
    root: Component<T>,
    defaults: T,
    draw_target: &'a mut Draw565,
}

impl<'a, T> App<'a, T>
where
    T: Copy,
{
    pub fn new(root: Component<T>, defaults: T, draw_target: &'a mut Draw565) -> Self {
        Self {
            root,
            defaults,
            draw_target,
        }
    }
}

impl<'a, T> Runner for App<'a, T>
where
    T: Copy,
{
    fn to_string(&self) -> String {
        let t = (self.defaults,);
        let r = self.root.call(t);
        r.to_string()
    }

    fn render(&mut self, size: Size) {
        let _ = self.draw_target.fill_solid(
            &Rectangle {
                top_left: Point { x: 10, y: 10 },
                size: size
                    - Size {
                        width: 20,
                        height: 20,
                    },
            },
            Rgb565::BLUE,
        );
    }
}

#[cfg(test)]
pub mod app_tests;
#[cfg(test)]
pub mod display_tests;
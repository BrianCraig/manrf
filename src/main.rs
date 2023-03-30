#![feature(fn_traits)]

use constraints::Constraints;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::Size;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::{Point, RgbColor};
use embedded_graphics::primitives::Rectangle;
use embedded_graphics_simulator::SimulatorDisplay;
use std::rc::Rc;

mod constraints;
mod render_tree;

use render_tree::{RenderData, RenderNode};

fn main() {}

pub trait Leaf {
    fn to_string(&self) -> String;
    fn render(&self, constraints: constraints::Constraints, element: Option<Element>) -> RenderNode;
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

    fn render(&self, constraints: constraints::Constraints, _: Option<Element>) -> RenderNode {
        // we keep the constraints from the parent;

        let mut sum = 0_u32;
        let mut max_cross = 0_u32;

        let render_child: Vec<_> = self
            .items
            .iter()
            .map(|item| item.render(constraints, Some(item.clone())))
            .map(|item| match item {
                RenderNode::SingleChild(data) => {
                    let new_offset = Point {
                        x: 0,
                        y: sum as i32,
                    };
                    sum += data.size.height;
                    max_cross = max_cross.max(data.size.width);
                    RenderNode::SingleChild(RenderData {
                        offset: new_offset,
                        ..data
                    })
                }
                RenderNode::MultiChild {
                    offset: _,
                    size,
                    child,
                } => {
                    let new_offset = Point {
                        x: 0,
                        y: sum as i32,
                    };
                    sum += size.height;
                    max_cross = max_cross.max(size.width);
                    RenderNode::MultiChild {
                        offset: new_offset,
                        size,
                        child,
                    }
                }
                RenderNode::Leaf => RenderNode::Leaf,
            })
            .collect();

        let size = Size {
            width: max_cross,
            height: sum,
        };
        let offset = (constraints.max - size) / 2;
        let offset = Point::new(offset.width as i32, offset.height as i32);
        RenderNode::MultiChild {
            offset,
            size,
            child: render_child,
        }
    }

    fn paint(&self, pos: Point, display: &mut Draw565) {
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

    fn render(&self, constraints: constraints::Constraints, element: Option<Element>) -> RenderNode {
        RenderNode::SingleChild(RenderData {
            offset: Point::default(),
            size: self.size,
            renderer: element.unwrap(),
            child: std::boxed::Box::new(RenderNode::Leaf),
        })
    }

    fn paint(&self, pos: Point, display: &mut Draw565) {
        let _ = display.fill_solid(&Rectangle { top_left:pos, size: self.size }, Rgb565::BLUE);
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

    fn render(&self, constraints: constraints::Constraints, element: Option<Element>) -> RenderNode {
        todo!();
    }

    fn paint(&self, pos: Point, display: &mut Draw565) {
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

    fn render(&self, constraints: constraints::Constraints, element: Option<Element>) -> RenderNode {
        todo!();
    }

    fn paint(&self, pos: Point, display: &mut Draw565) {
        todo!()
    }
}

type Component<T> = fn(T) -> Element;

type Draw565 = SimulatorDisplay<Rgb565>;

trait Runner {
    fn to_string(&self) -> String;
    fn render(&self, size: Size) -> RenderNode;
    fn paint(&mut self, node: RenderNode, offset: Point);
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

    fn render(&self, size: Size) -> RenderNode {
        let t = (self.defaults,);
        let r = self.root.call(t);
        r.render(constraints::Constraints::up_to(size), Some(r.clone()))
    }

    fn paint(&mut self, node: RenderNode, origin_offset: Point) {
        match node {
            RenderNode::SingleChild(RenderData {
                offset,
                size,
                renderer, 
                child,
            }) => {
                let new_offset = origin_offset + offset;
                renderer.paint(new_offset, &mut self.draw_target);
                self.paint(*child, new_offset);
            }
            RenderNode::MultiChild {
                offset,
                size,
                child,
            } => {
                let new_offset = origin_offset + offset;
                for item in child {
                    self.paint(item, new_offset);
                }
            }
            RenderNode::Leaf => {},
        }
    }
}

#[cfg(test)]
pub mod app_tests;
#[cfg(test)]
pub mod display_tests;

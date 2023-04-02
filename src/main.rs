#![feature(fn_traits)]

use constraints::Constraints;
use data_binding::GlobalStore;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::Size;
use embedded_graphics::mono_font::{ascii::FONT_6X10, MonoTextStyle};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::{Point, RgbColor};
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::Drawable;
use embedded_graphics_simulator::SimulatorDisplay;
use example_components::ComponentDefinition;

use std::rc::Rc;

mod constraints;
mod data_binding;
mod render_tree;
pub mod event;
mod example_components;
mod event_from_simulator;
mod testing_helpers;

use render_tree::{RenderData, RenderNode};

fn main() {}

pub trait Leaf {
    fn to_string(&self) -> String;
    fn render(&self, constraints: constraints::Constraints) -> (Size, RenderNode);
    fn paint(&self, pos: Point, display: &mut Draw565);
}

type Element = Rc<dyn Leaf>;

pub struct ListSelector {
    items: Vec<Element>,
    selected: usize,
}

impl ListSelector {
    pub fn new(items: Vec<Element>, selected: usize) -> Rc<Self> {
        Rc::new(ListSelector { items, selected })
    }
}

impl Leaf for ListSelector {
    fn to_string(&self) -> String {
        self.items[self.selected].to_string()
    }

    fn render(&self, constraints: constraints::Constraints) -> (Size, RenderNode) {
        let (size, render_node) = self.items[self.selected].render(constraints);
        (
            size,
            RenderNode::SingleChild(RenderData {
                offset: Point::zero(),
                child: std::boxed::Box::new(render_node),
                renderer: self.items[self.selected].clone(),
                size,
            }),
        )
    }

    fn paint(&self, pos: Point, display: &mut Draw565) {
        self.items[self.selected].paint(pos, display)
    }
}

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

    fn render(&self, constraints: constraints::Constraints) -> (Size, RenderNode) {
        // we keep the constraints from the parent;

        let mut sum = 0_u32;
        let mut max_cross = 0_u32;

        let render_child: Vec<_> = self
            .items
            .iter()
            .map(|item| (item, item.render(constraints)))
            .map(|(comp, (size, render_node))| {
                let new_offset = Point {
                    x: 0,
                    y: sum as i32,
                };
                sum += size.height;
                max_cross = max_cross.max(size.width);
                RenderNode::SingleChild(RenderData {
                    offset: new_offset,
                    child: std::boxed::Box::new(render_node),
                    renderer: comp.clone(),
                    size,
                })
            })
            .collect();

        let size = Size {
            width: max_cross,
            height: sum,
        };
        let offset = (constraints.max - size) / 2;
        let offset = Point::new(offset.width as i32, offset.height as i32);
        (
            size,
            RenderNode::MultiChild {
                offset,
                size,
                child: render_child,
            },
        )
    }

    fn paint(&self, _pos: Point, _display: &mut Draw565) {
        todo!()
    }
}

pub struct Box {
    size: Size,
    color: Rgb565,
}

impl Box {
    pub fn exactly(size: Size, color: Rgb565, _: Option<i32>) -> Rc<Self> {
        Rc::new(Self { size, color })
    }
}

impl Leaf for Box {
    fn to_string(&self) -> String {
        todo!()
    }

    fn render(&self, _constraints: constraints::Constraints) -> (Size, RenderNode) {
        (self.size, RenderNode::Leaf)
    }

    fn paint(&self, pos: Point, display: &mut Draw565) {
        let _ = display.fill_solid(
            &Rectangle {
                top_left: pos,
                size: self.size,
            },
            self.color,
        );
    }
}

pub struct Padding {
    padding: Size,
    child: Element,
}

impl Padding {
    pub fn new(padding: Size, child: Element) -> Rc<Self> {
        Rc::new(Self { padding, child })
    }
}

impl Leaf for Padding {
    fn to_string(&self) -> String {
        todo!()
    }

    fn render(&self, constraints: Constraints) -> (Size, RenderNode) {
        let double_padding = self.padding * 2;
        let child_constraints = constraints::Constraints {
            min: constraints.min + double_padding,
            max: constraints.max - double_padding,
        };
        let child = self.child.render(child_constraints);

        let offset = Point::new(self.padding.width as i32, self.padding.height as i32);
        let this_size = child.0 + double_padding;
        (
            this_size,
            RenderNode::SingleChild(RenderData {
                offset,
                size: child.0,
                renderer: self.child.clone(),
                child: std::boxed::Box::new(child.1),
            }),
        )
    }

    fn paint(&self, _pos: Point, _display: &mut Draw565) {}
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

    fn render(&self, _constraints: constraints::Constraints) -> (Size, RenderNode) {
        (Size::new(50, 10), RenderNode::Leaf)
    }

    fn paint(&self, pos: Point, display: &mut Draw565) {
        let mut small_style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);
        small_style.underline_color = embedded_graphics::text::DecorationColor::Custom(Rgb565::RED);
        small_style.background_color = Some(Rgb565::GREEN);
        let _ = embedded_graphics::text::Text::new(
            self.val,
            pos + Point::new(0, small_style.font.baseline as i32),
            small_style,
        )
        .draw(display);
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

    fn render(
        &self,
        _constraints: constraints::Constraints,
    ) -> (Size, RenderNode) {
        (Size::new(50, 10), RenderNode::Leaf)
    }

    fn paint(&self, pos: Point, display: &mut Draw565) {
        let mut small_style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);
        small_style.underline_color = embedded_graphics::text::DecorationColor::Custom(Rgb565::RED);
        small_style.background_color = Some(Rgb565::GREEN);
        let _ = embedded_graphics::text::Text::new(
            self.val.to_string().as_str(),
            pos + Point::new(0, small_style.font.baseline as i32),
            small_style,
        )
        .draw(display);
    }
}

type Component<T> = fn(T) -> Element;

type Draw565 = SimulatorDisplay<Rgb565>;

pub trait Runner {
    fn to_string(&mut self) -> String;
    fn render(&mut self, size: Size) -> RenderNode;
    fn paint(&mut self, node: RenderNode, target:&mut Draw565, offset: Point);
}

pub struct App
{
    store: GlobalStore,
    root: ComponentDefinition,
}

impl App
{
    pub fn new(root:ComponentDefinition) -> Self {
        Self {
            root,
            store: GlobalStore::new(),
        }
    }

    fn handle_event(&mut self, event: event::Event) {
        self.root.run_event_listener(&mut self.store, event);
    }
}

impl Runner for App
{
    fn to_string(&mut self) -> String {
        self.root.render(&mut self.store).to_string()
    }

    fn render(&mut self, size: Size) -> RenderNode {
        self.root.render(&mut self.store).render(constraints::Constraints::up_to(size)).1
    }

    fn paint(&mut self, node: RenderNode,target:&mut Draw565,  origin_offset: Point) {
        match node {
            RenderNode::SingleChild(RenderData {
                offset,
                size: _,
                renderer,
                child,
            }) => {
                let new_offset = origin_offset + offset;
                renderer.paint(new_offset, target);
                self.paint(*child, target, new_offset);
            }
            RenderNode::MultiChild {
                offset,
                size: _,
                child,
            } => {
                let new_offset = origin_offset + offset;
                for item in child {
                    self.paint(item, target, new_offset);
                }
            }
            RenderNode::Leaf => {}
        }
    }
}

#[cfg(test)]
pub mod app_tests;
#[cfg(test)]
pub mod display_tests;

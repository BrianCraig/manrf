#![feature(fn_traits)]

use crate::utils::*;
use embedded_graphics::mono_font::{ascii::FONT_6X10, MonoTextStyle};
use embedded_graphics_simulator::SimulatorDisplay;
use example_components::ComponentDefinition;

use std::rc::Rc;

mod constraints;
mod data_binding;
pub mod event;
mod event_from_simulator;
mod example_components;
mod full_example_test;
mod lol;
mod render_tree;
mod testing_helpers;
mod utils;
mod elements;
mod defs;

use render_tree::RenderNode;

fn main() {}

pub trait ElementTrait<S> {
    fn to_string(&self) -> String {
        todo!()
    }
    fn render(&self, constraints: constraints::Constraints, state: &S) -> (Size, RenderNode<S>);
    fn paint(&self, _pos: Point, _display: &mut Draw565) {}
    fn event_handler(&self, state: &mut S, event: Event) -> bool {
        false
    }
}

type Element<S> = Rc<dyn ElementTrait<S>>;

pub struct ListSelector<S> {
    items: Vec<Element<S>>,
    selected: usize,
}

impl<S> ListSelector<S> {
    pub fn new(items: Vec<Element<S>>, selected: usize) -> Rc<Self> {
        Rc::new(ListSelector { items, selected })
    }
}

impl<S> ElementTrait<S> for ListSelector<S> {
    fn to_string(&self) -> String {
        self.items[self.selected].to_string()
    }

    fn render(&self, constraints: constraints::Constraints, state: &S) -> (Size, RenderNode<S>) {
        let (size, render_node) = self.items[self.selected].render(constraints, state);
        (
            size,
            RenderNode::SingleChild {
                offset: Point::zero(),
                child: std::boxed::Box::new(render_node),
                renderer: self.items[self.selected].clone(),
                size,
            },
        )
    }

    fn paint(&self, pos: Point, display: &mut Draw565) {
        self.items[self.selected].paint(pos, display)
    }
}

pub struct Stack<S> {
    items: Vec<Element<S>>,
}

impl<S> Stack<S> {
    pub fn col(items: Vec<Element<S>>) -> Rc<Self> {
        Rc::new(Stack { items })
    }
}

impl<S> ElementTrait<S> for Stack<S> {
    fn to_string(&self) -> String {
        let coll = self
            .items
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        format!("[{}]", coll)
    }

    fn render(&self, constraints: constraints::Constraints, state: &S) -> (Size, RenderNode<S>) {
        // we keep the constraints from the parent;

        let mut sum = 0_u32;
        let mut max_cross = 0_u32;

        let render_child: Vec<_> = self
            .items
            .iter()
            .map(|item| (item, item.render(constraints, state)))
            .map(|(comp, (size, render_node))| {
                let new_offset = Point {
                    x: 0,
                    y: sum as i32,
                };
                sum += size.height;
                max_cross = max_cross.max(size.width);
                RenderNode::SingleChild {
                    offset: new_offset,
                    child: std::boxed::Box::new(render_node),
                    renderer: comp.clone(),
                    size,
                }
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
}

pub struct Box<S> {
    size: Size,
    color: Rgb565,
    child: Option<Element<S>>,
}

impl<S> Box<S> {
    pub fn exactly(size: Size, color: Rgb565, child: Option<Element<S>>) -> Rc<Self> {
        Rc::new(Self { size, color, child })
    }
}

impl<S> ElementTrait<S> for Box<S> {
    fn render(&self, constraints: constraints::Constraints, state: &S) -> (Size, RenderNode<S>) {
        (
            self.size,
            match &self.child {
                Some(child) => {
                    let (size, render_node) = child.render(
                        constraints::Constraints {
                            min: self.size,
                            max: self.size,
                        },
                        state,
                    );
                    RenderNode::SingleChild {
                        offset: Point::zero(),
                        child: std::boxed::Box::new(render_node),
                        renderer: child.clone(),
                        size,
                    }
                }
                None => RenderNode::Leaf,
            },
        )
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

pub struct Padding<S> {
    padding: Size,
    child: Element<S>,
}

impl<S> Padding<S> {
    pub fn new(padding: Size, child: Element<S>) -> Rc<Self> {
        Rc::new(Self { padding, child })
    }
}

impl<S> ElementTrait<S> for Padding<S> {
    fn render(&self, constraints: constraints::Constraints, state: &S) -> (Size, RenderNode<S>) {
        let double_padding = self.padding * 2;
        let child_constraints = constraints::Constraints {
            min: constraints.min + double_padding,
            max: constraints.max - double_padding,
        };
        let child = self.child.render(child_constraints, state);

        let offset = Point::new(self.padding.width as i32, self.padding.height as i32);
        let this_size = child.0 + double_padding;
        (
            this_size,
            RenderNode::SingleChild {
                offset,
                size: child.0,
                renderer: self.child.clone(),
                child: std::boxed::Box::new(child.1),
            },
        )
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

impl<S> ElementTrait<S> for Text {
    fn to_string(&self) -> String {
        self.val.to_string()
    }

    fn render(&self, _constraints: constraints::Constraints, _state: &S) -> (Size, RenderNode<S>) {
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

impl<S> ElementTrait<S> for Number {
    fn to_string(&self) -> String {
        self.val.to_string()
    }

    fn render(&self, _constraints: constraints::Constraints, _state: &S) -> (Size, RenderNode<S>) {
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

pub struct Border<S> {
    color: Rgb565,
    child: Element<S>,
    size: u8,
}

impl<S> Border<S> {
    pub fn bottom(size: u8, color: Rgb565, child: Element<S>) -> Rc<Self> {
        Rc::new(Self { color, child, size })
    }
}

impl<S> ElementTrait<S> for Border<S> {
    fn render(&self, constraints: constraints::Constraints, state: &S) -> (Size, RenderNode<S>) {
        let child = self.child.render(constraints, state);
        let this_size = child.0 + Size::new(0, self.size as u32);
        (
            this_size,
            RenderNode::SingleChild {
                offset: Point::new(0, self.size as i32),
                size: child.0,
                renderer: self.child.clone(),
                child: std::boxed::Box::new(child.1),
            },
        )
    }

    fn paint(&self, pos: Point, display: &mut Draw565) {
        let _ = display.fill_solid(
            &Rectangle {
                top_left: pos,
                size: Size::new(100, self.size as u32),
            },
            self.color,
        );
    }
}

#[derive(Default, Clone)]
pub struct ItemSelectorState {
    active: usize,
    selected: Option<usize>,
}
pub struct ItemSelector<S, T> {
    items_lookup: fn(&S) -> &Vec<T>,
    selector_state_lookup: fn(&S) -> ItemSelectorState,
    set_selector_state: fn(&mut S, ItemSelectorState),
    render_item: fn(&T, bool) -> Element<S>,
}

impl<S, T> ItemSelector<S, T> {
    pub fn new(
        items_lookup: fn(&S) -> &Vec<T>,
        selector_state_lookup: fn(&S) -> ItemSelectorState,
        set_selector_state: fn(&mut S, ItemSelectorState),
        render_item: fn(&T, bool) -> Element<S>,
    ) -> Rc<Self> {
        Rc::new(Self {
            items_lookup,
            selector_state_lookup,
            set_selector_state,
            render_item,
        })
    }
}

impl<S, T> ElementTrait<S> for ItemSelector<S, T> {
    fn render(&self, constraints: constraints::Constraints, state: &S) -> (Size, RenderNode<S>) {
        let mut size = Size::new(0, 0);
        let mut children = Vec::new();
        let items = (self.items_lookup)(state);
        let element_state = (self.selector_state_lookup)(state);
        for (index, item) in items.iter().enumerate() {
            let active = index == element_state.active;
            let render = (self.render_item)(item, active);
            let child = render.render(constraints, state);
            children.push(RenderNode::SingleChild {
                offset: Point::new(0, size.height as i32),
                size: child.0,
                renderer: render,
                child: std::boxed::Box::new(child.1),
            });
            size.width = size.width.max(child.0.width);
            size.height += child.0.height;
        }
        (
            size,
            RenderNode::MultiChild {
                offset: Point::new(0, 0),
                size,
                child: children,
            },
        )
    }

    fn event_handler(&self, state: &mut S, event: event::Event) -> bool {
        let mut element_state = (self.selector_state_lookup)(state);
        match event {
            Event::DirectionPressed(Direction::Up) => {
                if element_state.active > 0 {
                    element_state.active -= 1;
                    (self.set_selector_state)(state, element_state);
                }
                true
            }
            Event::DirectionPressed(Direction::Down) => {
                let items = (self.items_lookup)(state);
                if element_state.active < items.len() - 1 {
                    element_state.active += 1;
                    (self.set_selector_state)(state, element_state);
                }
                true
            }
            Event::ButtonPressed(Button::Principal) => {
                element_state.selected = Some(element_state.active);
                (self.set_selector_state)(state, element_state);
                true
            }
            _ => false,
        }
    }
}

type Draw565 = SimulatorDisplay<Rgb565>;

pub trait Runner<S> {
    fn to_string(&mut self) -> String;
    fn render(&mut self, size: Size) -> RenderNode<S>;
    fn paint(&mut self, node: &RenderNode<S>, target: &mut Draw565, offset: Point);
}

pub struct App<T>
where
    T: Default,
{
    state: T,
    root: ComponentDefinition<T>,
}

impl<T> App<T>
where
    T: Default,
{
    pub fn new(root: ComponentDefinition<T>) -> Self {
        Self {
            root,
            state: T::default(),
        }
    }

    fn handle_event(&mut self, event: event::Event, render_root: &RenderNode<T>) {
        if self.root.run_event_listener(&mut self.state, event.clone()) {
            return;
        }
        if !self.handle_event_recursive(event.clone(), render_root){
            println!("Unhandled event: {:?}", event);
        }
    }

    fn handle_event_recursive(&mut self, event: event::Event, render_root: &RenderNode<T>) -> bool {
        match render_root {
            RenderNode::SingleChild {
                offset: _,
                size: _,
                renderer,
                child,
            } => {
                if renderer.event_handler(&mut self.state, event.clone()) {
                    return true;
                }
                self.handle_event_recursive(event, child)
            }
            RenderNode::MultiChild {
                offset: _,
                size: _,
                child,
            } => {
                child.iter().any(|c| self.handle_event_recursive(event.clone(), c))
            },
            RenderNode::Leaf => {false}
        }
    }
}

impl<T> Runner<T> for App<T>
where
    T: Default,
{
    fn to_string(&mut self) -> String {
        self.root.render(&mut self.state).to_string()
    }

    fn render(&mut self, size: Size) -> RenderNode<T> {
        self.root
            .render(&mut self.state)
            .render(constraints::Constraints::up_to(size), &self.state)
            .1
    }

    fn paint(&mut self, node: &RenderNode<T>, target: &mut Draw565, origin_offset: Point) {
        match node {
            RenderNode::SingleChild {
                offset,
                size: _,
                renderer,
                child,
            } => {
                let new_offset = origin_offset + offset.clone();
                renderer.paint(new_offset, target);
                self.paint(child, target, new_offset);
            }
            RenderNode::MultiChild {
                offset,
                size: _,
                child,
            } => {
                let new_offset = origin_offset + offset.clone();
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

use defs::*;
use embedded_graphics::mono_font::{ascii::FONT_6X10, MonoTextStyle};
use utils::*;

pub mod defs;
pub mod elements;
pub mod event;

pub mod palette;
mod testing_helpers;
pub mod utils;

pub struct Stack<'a, S, T> {
    items: Vec<Element<'a, S, T>>,
}

impl<'a, S: Default, T> Stack<'a, S, T> {
    pub fn col(items: Vec<Element<'a, S, T>>) -> Rc<Self> {
        Rc::new(Stack { items })
    }
}

impl<'a, S: Default, T: DrawTarget<Color = Rgb888>> ElementTrait<'a, S, T> for Stack<'a, S, T> {
    fn to_string(&self) -> String {
        let coll = self
            .items
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        format!("[{}]", coll)
    }

    fn render(&self, constraints: Constraints, state: &S) -> (Size, RenderNode<'a, S, T>) {
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
        let size = constraints.clamp(&size);
        (
            size,
            RenderNode::MultiChild {
                offset: Point::default(),
                size,
                child: render_child,
            },
        )
    }
}

pub struct Box<'a, S, T> {
    size: Size,
    color: Rgb565,
    child: Option<Element<'a, S, T>>,
}

impl<'a, S, T> Box<'a, S, T> {
    pub fn exactly(size: Size, color: Rgb565, child: Option<Element<'a, S, T>>) -> Rc<Self> {
        Rc::new(Self { size, color, child })
    }
}

impl<'a, S: Default, T: DrawTarget<Color = Rgb888>> ElementTrait<'a, S, T> for Box<'a, S, T> {
    fn render(&self, _constraints: Constraints, state: &S) -> (Size, RenderNode<'a, S, T>) {
        (
            self.size,
            match &self.child {
                Some(child) => {
                    let (size, render_node) = child.render(
                        Constraints {
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

    fn paint(&self, _size: Size, pos: Point, display: &mut T) {
        let _ = display.fill_solid(
            &Rectangle {
                top_left: pos,
                size: self.size,
            },
            self.color.into(),
        );
    }
}

pub struct Text {
    val: String,
}

impl Text {
    pub fn new(val: String) -> Rc<Self> {
        Rc::new(Self { val })
    }
}

impl<'a, S: Default, T: DrawTarget<Color = Rgb888>> ElementTrait<'a, S, T> for Text {
    fn to_string(&self) -> String {
        self.val.to_string()
    }

    fn render(&self, constraints: Constraints, _state: &S) -> (Size, RenderNode<'a, S, T>) {
        (
            constraints.clamp(&Size::new(self.val.len() as u32 * 6, 10)),
            RenderNode::Leaf,
        )
    }

    fn paint(&self, _size: Size, pos: Point, display: &mut T) {
        let mut small_style = MonoTextStyle::new(&FONT_6X10, Rgb888::WHITE);
        small_style.underline_color = embedded_graphics::text::DecorationColor::Custom(Rgb888::RED);
        small_style.background_color = Some(Rgb888::GREEN);
        let _ = embedded_graphics::text::Text::new(
            self.val.as_str(),
            pos + Point::new(0, small_style.font.baseline as i32),
            small_style,
        )
        .draw(display);
    }
}

#[derive(Default, Clone)]
pub struct ItemSelectorState {
    pub active: usize,
    pub selected: Option<usize>,
}
pub struct ItemSelector<S, T, V> {
    items_lookup: fn(&S) -> &Vec<V>,
    selector_state_lookup: fn(&S) -> ItemSelectorState,
    set_selector_state: fn(&mut S, ItemSelectorState),
    render_item: fn(&V, bool) -> Element<S, T>,
}

impl<S, T, V> ItemSelector<S, T, V> {
    pub fn new(
        items_lookup: fn(&S) -> &Vec<V>,
        selector_state_lookup: fn(&S) -> ItemSelectorState,
        set_selector_state: fn(&mut S, ItemSelectorState),
        render_item: fn(&V, bool) -> Element<S, T>,
    ) -> Rc<Self> {
        Rc::new(Self {
            items_lookup,
            selector_state_lookup,
            set_selector_state,
            render_item,
        })
    }
}

impl<'a,S: Default, T: DrawTarget<Color = Rgb888>, V> ElementTrait<'a,S, T> for ItemSelector<S, T, V> {
    fn render(&self, constraints: Constraints, state: &S) -> (Size, RenderNode<'a,S, T>) {
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

pub struct App<'a, S: Default, T> {
    state: S,
    root: Element<'a, S, T>,
    last_render_tree: RenderNode<'a, S, T>,
    inital_size: Size,
    pub target: T,
}

impl<'a, S: Default, T: DrawTarget<Color = Rgb888> + 'static> App<'a, S, T> {
    pub fn new(root: ComponentGenerator<S, T>, inital_size: Size, target: T) -> Self {
        let state = S::default();
        let root: Rc<elements::Component<S, T>> = crate::elements::Component::new(root);
        let last_render_tree = root
            .render(
                Constraints {
                    min: Size::zero(),
                    max: inital_size,
                },
                &state,
            )
            .1;

        Self {
            root,
            state,
            inital_size,
            last_render_tree,
            target,
        }
    }

    fn handle_event_recursive(
        &mut self,
        event: event::Event,
        render_root: &RenderNode<S, T>,
    ) -> bool {
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
            } => child
                .iter()
                .any(|c| self.handle_event_recursive(event.clone(), c)),
            RenderNode::Leaf => false,
        }
    }

    fn paint(node: &RenderNode<S, T>, target: &mut T, origin_offset: Point) {
        match node {
            RenderNode::SingleChild {
                offset,
                size,
                renderer,
                child,
            } => {
                let new_offset = origin_offset + offset.clone();
                renderer.paint(*size, new_offset, target);
                Self::paint(child, target, new_offset);
            }
            RenderNode::MultiChild {
                offset,
                size: _,
                child,
            } => {
                let new_offset = origin_offset + offset.clone();
                for item in child {
                    Self::paint(item, target, new_offset);
                }
            }
            RenderNode::Leaf => {}
        }
    }
}

impl<'a,S: Default, T: DrawTarget<Color = Rgb888> + 'static> Runner for App<'a,S, T> {
    fn to_string(&mut self) -> String {
        self.root.to_string()
    }

    fn handle_event(&mut self, event: event::Event) {
        let mut swap_tree = RenderNode::Leaf;
        core::mem::swap(&mut swap_tree, &mut self.last_render_tree);
        if self.handle_event_recursive(event.clone(), &swap_tree) {
            self.last_render_tree = self
                .root
                .render(
                    Constraints {
                        min: Size::zero(),
                        max: self.inital_size,
                    },
                    &self.state,
                )
                .1;
        } else {
            core::mem::swap(&mut swap_tree, &mut self.last_render_tree);
            println!("Unhandled event: {:?}", event);
        }
    }

    fn draw(&mut self) {
        Self::paint(&self.last_render_tree, &mut self.target, Point::new(0, 0));
    }
}

#[cfg(test)]
mod full_example_test;

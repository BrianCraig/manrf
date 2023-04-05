use std::{
    any::{Any, TypeId},
    rc::Rc,
};

use embedded_graphics::prelude::{Point, Size};

use crate::{constraints::Constraints, event::Event};
/// We have two types of components, statefull and stateless, and we want to be able to render them both.
/// Stateless components always return the same thing, and they don't have any state.
/// Statefull components have state, and they can change their state based on events.

trait Layout {
    fn layout_child(&self, constraints: Constraints) -> (Size, NodeDescription);
}

trait Renderizable {
    fn render(&self, size: Size, offset: Point);
}

trait Statefull {
    /// Handle an event, and return true if the event should stop it's propagation.
    fn handle_event(&mut self, event: Event) -> bool;
    /// consumes the other state and updates the current state.
    fn digest_state(&mut self, other: Element);
}

trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

trait StatelessWidgetTrait: AsAny + Layout + Any {}
trait StatefullWidgetTrait: AsAny + Layout + Statefull + Any {}

type StatelessWidget = Rc<dyn StatelessWidgetTrait>;

type StatefullWidget = Rc<dyn StatefullWidgetTrait>;

enum NodeDescription {
    SingleChild {
        offset: Point,
        size: Size,
        element: Element,
        child: Box<NodeDescription>,
    },
    MultiChild {
        offset: Point,
        element: Element,
        child: Vec<NodeDescription>,
    },
    Leaf,
}

enum Element {
    Statefull(StatefullWidget),
    Stateless(StatelessWidget),
}

impl Layout for Element {
    fn layout_child(&self, constraints: Constraints) -> (Size, NodeDescription) {
        match self {
            Element::Statefull(statefull) => statefull.layout_child(constraints),
            Element::Stateless(stateless) => stateless.layout_child(constraints),
        }
    }
}

struct SimpleChildWidget {
    child: Element,
}

impl Layout for SimpleChildWidget {
    fn layout_child(&self, constraints: Constraints) -> (Size, NodeDescription) {
        let (size, child) = self.child.layout_child(constraints);
        (
            size,
            NodeDescription::SingleChild {
                offset: Point::default(),
                size,
                element: todo!(),
                child: Box::new(child),
            },
        )
    }
}

impl StatelessWidgetTrait for SimpleChildWidget {}

impl AsAny for SimpleChildWidget {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct SimpleStateWidget {
    state: i32,
    child: Element,
}

impl Layout for SimpleStateWidget {
    fn layout_child(&self, constraints: Constraints) -> (Size, NodeDescription) {
        let (size, child) = self.child.layout_child(constraints);
        (
            size,
            NodeDescription::SingleChild {
                offset: Point::default(),
                size,
                element: todo!(),
                child: Box::new(child),
            },
        )
    }
}

impl Statefull for SimpleStateWidget {
    fn handle_event(&mut self, event: Event) -> bool {
        if let Event::ButtonPressed(_) = event {
            self.state += 1;
            true
        } else {
            false
        }
    }

    fn digest_state(&mut self, other: Element) {
        if let Element::Statefull(statefull) = other {
            if let Some(other_state) = (*statefull).as_any().downcast_ref::<SimpleStateWidget>() {
                self.state = other_state.state;
            }
        }
    }
}

impl AsAny for SimpleStateWidget {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl StatefullWidgetTrait for SimpleStateWidget {}

#[derive(Debug)]
struct TypedData {
    data: Box<dyn Any>,
    type_id: TypeId,
}

impl PartialEq for TypedData {
    fn eq(&self, other: &Self) -> bool {
        self.type_id == other.type_id
    }
}

#[derive(Debug, PartialEq)]
enum ElementTree {
    DescriptionWidget {
        params: TypedData,
        child: Box<ElementTree>,
    },
    StatefullWidget {
        params: TypedData,
    },
    Leaf,
}

struct Decoration {}

impl Decoration {
    fn new(params: TypedData, child: ElementTree) -> ElementTree {
        ElementTree::DescriptionWidget {
            params,
            child: Box::new(child),
        }
    }

    fn border_all(width: u32) -> TypedData {
        TypedData {
            data: Box::new(width),
            type_id: TypeId::of::<u32>(),
        }
    }
}

#[test]
fn statefull_widget_gen() {
    let statefull = |state: &u32| -> ElementTree {
        Decoration::new(
            Decoration::border_all(*state + 1),
            Decoration::new(Decoration::border_all(*state + 1), ElementTree::Leaf),
        )
    };

    let expected_tree = ElementTree::DescriptionWidget {
        params: Decoration::border_all(2),
        child: Box::new(ElementTree::DescriptionWidget {
            params: Decoration::border_all(2),
            child: Box::new(ElementTree::Leaf),
        }),
    };

    assert_eq!(statefull(&1), expected_tree);

    println!("{:?}", statefull(&1));
}

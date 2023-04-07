

use embedded_graphics::prelude::Size;

use crate::{
    event::{Event, Button},
    Element, testing_helpers::test_in_window, Number,
};

type EventFunction<T> = fn(&mut T, Event) -> bool;

type ComponentGenerator<T> = fn(&mut T) -> Element<T>;
pub struct ComponentDefinition<T> {
    generator: ComponentGenerator<T>,
    events_listener: Option<EventFunction<T>>,
}

impl<T> ComponentDefinition<T> {
    pub fn new(generator: ComponentGenerator<T>) -> Self {
        Self {
            generator,
            events_listener: None,
        }
    }

    pub fn render(&self, store: &mut T) -> Element<T> {
        (self.generator)(store)
    }

    pub fn with_events_listener(&mut self, events_listener: EventFunction<T>) {
        self.events_listener = Some(events_listener);
    }

    pub fn run_event_listener(&self, store: &mut T, event: Event) -> bool{
        if let Some(events_listener) = self.events_listener {
            events_listener(store, event);
        }
        false
    }
}

#[test]
fn component_definition() {
    #[derive(Default, Clone)]
    pub struct AppState {
        counter: i32,
    }

    let mut state = AppState::default();
    let mut component: ComponentDefinition<AppState> =
        ComponentDefinition::new(|state| Number::new(state.counter));

    component.events_listener = Some(|state, event| match event {
        Event::ButtonPressed(Button::Principal) => {
            state.counter += 1;
            true
        }
        _ => false,
    });

    assert_eq!(component.render(&mut state).to_string(), "0");
    component.run_event_listener(&mut state, Event::ButtonPressed(Button::Principal));
    assert_eq!(component.render(&mut state).to_string(), "1");
    component.run_event_listener(&mut state, Event::ButtonPressed(Button::Secondary));
    assert_eq!(component.render(&mut state).to_string(), "1");
}

#[test]
fn component_list_selector_event() {
    #[derive(Clone)]
    pub struct AppState {
        elements: Vec<i32>,
        selected: usize,
    }

    impl Default for AppState {
        fn default() -> Self {
            Self {
                elements: vec![1, 2, 3],
                selected: 0,
            }
        }
    }

    let mut state = AppState::default();
    let mut component: ComponentDefinition<AppState> = ComponentDefinition::new(|state| {
        crate::Stack::col(vec![
            crate::Text::new("Select an element"),
            crate::ListSelector::new(
                state
                    .elements
                    .iter()
                    .map(|element| crate::Number::new(*element) as Element<AppState>)
                    .collect(),
                state.selected,
            ),
        ])
    });

    component.events_listener = Some(|state, event| match event {
        Event::ButtonPressed(Button::Principal) => {
            state.selected = (state.selected + 1) % state.elements.len();
            true
        }
        _ => false,
    });

    assert_eq!(
        component.render(&mut state).to_string(),
        "[Select an element, 1]"
    );
    component.run_event_listener(&mut state, Event::ButtonPressed(Button::Principal));
    assert_eq!(
        component.render(&mut state).to_string(),
        "[Select an element, 2]"
    );
    component.run_event_listener(&mut state, Event::ButtonPressed(Button::Principal));
    assert_eq!(
        component.render(&mut state).to_string(),
        "[Select an element, 3]"
    );
    component.run_event_listener(&mut state, Event::ButtonPressed(Button::Principal));
    assert_eq!(
        component.render(&mut state).to_string(),
        "[Select an element, 1]"
    );
}

#[ignore]
#[test]
fn component_list_selector_manual() {
    #[derive(Clone)]
    pub struct AppState {
        elements: Vec<i32>,
        selected: usize,
    }

    impl Default for AppState {
        fn default() -> Self {
            Self {
                elements: vec![1, 2, 3],
                selected: 0,
            }
        }
    }

    let mut component:ComponentDefinition<AppState> = ComponentDefinition::new(|state| {
        crate::Stack::col(vec![
            crate::Text::new("selector"),
            crate::ListSelector::new(
                state
                    .elements
                    .iter()
                    .map(|element| crate::Number::new(*element) as Element<AppState>)
                    .collect(),
                state.selected,
            ),
        ])
    });


    component.events_listener = Some(|state, event| match event {
        Event::ButtonPressed(Button::Principal) => {
            state.selected = (state.selected + 1) % state.elements.len();
            true
        }
        _ => false,
    });

    test_in_window(Size::new(128, 128), component, |_, _| ());
}

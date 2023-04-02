use crate::{Element, event::{Event, Button}, data_binding::GlobalStore, Text, Number};

type EventFunction = fn(&mut GlobalStore, Event);

type ComponentGenerator = fn(&mut GlobalStore) -> Element;
pub struct ComponentDefinition {
    generator: ComponentGenerator,
    events_listener: Option<EventFunction>
}

impl ComponentDefinition {
    pub fn new(generator: ComponentGenerator) -> Self {
        Self {
            generator,
            events_listener: None
        }
    }

    pub fn render(&self, store: &mut GlobalStore) -> Element {
        (self.generator)(store)
    }

    pub fn with_events_listener(&mut self, events_listener: EventFunction) {
        self.events_listener = Some(events_listener);
    }

    pub fn run_event_listener(&self, store: &mut GlobalStore, event: Event) {
        if let Some(events_listener) = self.events_listener {
            events_listener(store, event);
        }
    }
}

#[test]
fn component_definition() {
    #[derive(Default, Clone)]
    pub struct AppState {
        counter: i32
    }

    let mut store = GlobalStore::new();
    let mut component = ComponentDefinition::new(|store| {
        Number::new(store.get::<AppState>().counter)
    });

    component.events_listener = Some(|store, event| {
        let mut state = store.get::<AppState>().clone();
        if let Event::ButtonPressed(Button::Principal) = event {
            state.counter += 1;
            store.insert(state);
        }
    });
    
    assert_eq!(component.render(&mut store).to_string(), "0");
    component.run_event_listener(&mut store, Event::ButtonPressed(Button::Principal));
    assert_eq!(component.render(&mut store).to_string(), "1");
    component.run_event_listener(&mut store, Event::ButtonPressed(Button::Secondary));
    assert_eq!(component.render(&mut store).to_string(), "1");
}

#[test]
fn component_list_selector_event() {
    #[derive(Default, Clone)]
    pub struct AppState {
        elements: Vec<i32>,
        selected: usize
    }

    let mut store = GlobalStore::new();
    let mut component = ComponentDefinition::new(|store| {
        let state = store.get::<AppState>();
        crate::Stack::col(vec![
            crate::Text::new("Select an element"),
            crate::ListSelector::new(state.elements.iter().map(|element| {
                crate::Number::new(*element) as Element
            }).collect(), state.selected)
        ])
    });

    component.events_listener = Some(|store, event| {
        let mut state = store.get::<AppState>().clone();
        if let Event::ButtonPressed(Button::Principal) = event {
            state.selected = (state.selected + 1) % state.elements.len();
            store.insert(state);
        }
    });

    store.insert(AppState {
        elements: vec![1, 2, 3],
        selected: 0
    });

    assert_eq!(component.render(&mut store).to_string(), "[Select an element, 1]");
    component.run_event_listener(&mut store, Event::ButtonPressed(Button::Principal));
    assert_eq!(component.render(&mut store).to_string(), "[Select an element, 2]");
    component.run_event_listener(&mut store, Event::ButtonPressed(Button::Principal));
    assert_eq!(component.render(&mut store).to_string(), "[Select an element, 3]");
    component.run_event_listener(&mut store, Event::ButtonPressed(Button::Principal));
    assert_eq!(component.render(&mut store).to_string(), "[Select an element, 1]");
}
use crate::defs::ComponentGenerator;
use crate::elements;
use crate::elements::{BorderDefinition, StyleDefinition};
use crate::palette::PALETTE_DREAM;
use crate::utils::*;

use crate::{ItemSelector, ItemSelectorState};
use embedded_graphics::prelude::Size;

const BORDERED_STYLE: StyleDefinition = StyleDefinition {
    background: Some(PALETTE_DREAM.darkest),
    margin: EdgeInsets::symmetric(2, 4),
    border: BorderDefinition::new(PALETTE_DREAM.dark, EdgeInsets::new(1, 2, 3, 4)),
    padding: EdgeInsets::all(2),
};

use crate::{testing_helpers::test_in_window, Element, Stack, Text};

#[derive(Clone)]
struct Key {
    text: String,
    key: u32,
}

impl Key {
    fn new(text: &str, key: u32) -> Self {
        Self {
            text: text.to_string(),
            key,
        }
    }
}

impl Default for Key {
    fn default() -> Self {
        Self {
            text: "Default".to_string(),
            key: 0,
        }
    }
}

struct AppState {
    keys: Vec<Key>,
    keys_selected_state: crate::ItemSelectorState,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            keys: vec![
                Key::new("First", 1),
                Key::new("Second", 2),
                Key::new("Third", 3),
            ],
            keys_selected_state: ItemSelectorState::default(),
        }
    }
}

static GO_BACK: elements::EventHandler<AppState> = |state, event| {
    if let Event::ButtonPressed(Button::Back) = event {
        state.keys_selected_state.selected = None;
        true
    } else {
        false
    }
};

static ITEM_SELECTOR_VIEW: elements::Generator<AppState> = |_| {
    ItemSelector::<AppState, Key>::new(
        |state| &state.keys,
        |state| state.keys_selected_state.clone(),
        |state, new_state| state.keys_selected_state = new_state,
        |key: &Key, selected: bool| {
            elements::Style::new_with_style(
                StyleDefinition {
                    background: if selected {
                        Some(PALETTE_DREAM.light)
                    } else {
                        Some(PALETTE_DREAM.darkest)
                    },
                    ..BORDERED_STYLE
                },
                Text::new(format!("Key: {} {}", key.text, key.key)),
            )
        },
    )
};

static SELECTED_VIEW: elements::Generator<AppState> = |state| {
    let selected_key: Key = state
        .keys_selected_state
        .selected
        .map(|index| &state.keys[index])
        .unwrap_or(&Key::default())
        .clone();
    elements::Handler::new(
        GO_BACK,
        elements::border(
            BorderDefinition {
                color: PALETTE_DREAM.darkest,
                size: EdgeInsets::all(2),
            },
            Text::new(format!(
                "Selected: {} {}",
                selected_key.text, selected_key.key
            )),
        ),
    )
};

static MAIN_MENU: ComponentGenerator<AppState> = |state| {
    let is_selected = state.keys_selected_state.selected.is_some();

    let actual_view = match is_selected {
        true => SELECTED_VIEW,
        false => ITEM_SELECTOR_VIEW,
    };

    elements::background(
        PALETTE_DREAM.darkest,
        elements::center(Stack::col(vec![
            elements::border(
                BorderDefinition {
                    color: PALETTE_DREAM.darkest,
                    size: EdgeInsets::all(2),
                },
                Text::new(if is_selected {
                    "Selected".to_string()
                } else {
                    "Not selected".to_string()
                }),
            ) as Element<AppState>,
            elements::Component::new(actual_view) as Element<AppState>,
        ])),
    )
};

#[ignore]
#[test]
fn create_keys_app() {
    test_in_window::<AppState>(Size::new(128, 128), MAIN_MENU, |_, _| ());
}

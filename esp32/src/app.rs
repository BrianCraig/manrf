use manrf::defs::{Element, State};
use manrf::event::{Button, Event};
use manrf::utils::EdgeInsets;
use manrf::{elements::*, palette::PALETTE_DREAM};
const BORDERED_STYLE: StyleDefinition = StyleDefinition {
    background: Some(PALETTE_DREAM.darkest),
    margin: EdgeInsets::symmetric(2, 4),
    border: BorderDefinition::new(PALETTE_DREAM.dark, EdgeInsets::new(1, 2, 3, 4)),
    padding: EdgeInsets::all(2),
};

use manrf::{ItemSelector, ItemSelectorState, Stack, Text};

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

pub struct AppState {
    keys: Vec<Key>,
    keys_selected_state: ItemSelectorState,
}

impl State for AppState {}

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

static GO_BACK: EventHandler<AppState> = |state, event| {
    if let Event::ButtonPressed(Button::Back) = event {
        state.keys_selected_state.selected = None;
        true
    } else {
        false
    }
};

fn item_selector_view(_state: &AppState) -> Element<AppState> {
    ItemSelector::<AppState, Key>::new(
        |state| &state.keys,
        |state| state.keys_selected_state.clone(),
        |state, new_state| state.keys_selected_state = new_state,
        |key: &Key, selected: bool| {
            Style::new_with_style(
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
}

fn selected_view(state: &AppState) -> Element<AppState> {
    let selected_key: Key = state
        .keys_selected_state
        .selected
        .map(|index| &state.keys[index])
        .unwrap_or(&Key::default())
        .clone();
    Handler::new(
        GO_BACK,
        border(
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
}

pub fn main_menu(state: &AppState) -> Element<AppState> {
    let is_selected = state.keys_selected_state.selected.is_some();
    let actual_view = if is_selected {
        selected_view
    } else {
        item_selector_view
    };

    background(
        PALETTE_DREAM.darkest,
        center(Stack::col(vec![
            border(
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
            Component::new(actual_view) as Element<AppState>,
        ])),
    )
}

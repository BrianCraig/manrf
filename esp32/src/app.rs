use display_interface_spi::SPIInterfaceNoCS;
use embedded_graphics::draw_target::ColorConverted;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::DrawTarget;
use esp_idf_hal::gpio::{PinDriver, Gpio16, Output, Gpio23};
use esp_idf_hal::spi::{SpiDeviceDriver, SpiDriver};
use manrf::defs::{ComponentGenerator, Element, State};
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

impl State for AppState{}

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

fn ITEM_SELECTOR_VIEW<T:DrawTarget<Color = Rgb888> + 'static>( state:&AppState) -> Element<AppState, T> {
    ItemSelector::<AppState,T,  Key>::new(
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

fn SELECTED_VIEW<T:DrawTarget<Color = Rgb888>+ 'static>( state:&AppState) -> Element<AppState, T>{
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

pub fn main_menu<T:DrawTarget<Color = Rgb888>+ 'static>(state: &AppState) -> Element<AppState, T>{
    let is_selected = state.keys_selected_state.selected.is_some();
    let actual_view = if is_selected {
        SELECTED_VIEW::<T>(state)
    } else {
        ITEM_SELECTOR_VIEW::<T>(state)
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
            ) as Element<AppState, T>,
            //Component::new(actual_view) as Element<AppState, T>,
        ])),
    )
}

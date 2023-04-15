#[ignore]
#[test]
fn create_keys_app() {
    use crate::elements;
    use crate::elements::{BorderDefinition, StyleDefinition};
    use crate::palette::PALETTE_DREAM;
    use crate::utils::*;

    use crate::{ItemSelector, ItemSelectorState};
    use embedded_graphics::prelude::Size;

    const bordered_style: StyleDefinition = StyleDefinition {
        background: Some(PALETTE_DREAM.darkest),
        margin: EdgeInsets::symmetric(2, 4),
        border: BorderDefinition::new(PALETTE_DREAM.dark, EdgeInsets::new(1, 2, 3, 4)),
        padding: EdgeInsets::all(2),
    };

    use crate::{
        component::ComponentDefinition, testing_helpers::test_in_window, Element, Stack, Text,
    };

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

    let main_menu: ComponentDefinition<AppState> = ComponentDefinition::new(|state| {
        let item_selector: std::rc::Rc<ItemSelector<AppState, Key>> = ItemSelector::new(
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
                        ..bordered_style
                    },
                    Text::new(format!("Key: {} {}", key.text, key.key)),
                )
            },
        );

        Stack::col(vec![
            elements::border(
                BorderDefinition {
                    color: PALETTE_DREAM.darkest,
                    size: EdgeInsets::all(2),
                },
                Text::new("Main Menu".to_string()),
            ) as Element<AppState>,
            item_selector as Element<AppState>,
        ])
    });

    test_in_window::<AppState>(Size::new(128, 128), main_menu, |_, _| ());
}

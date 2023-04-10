use crate::palette::PALETTE_DREAM;

#[ignore]
#[test]
fn create_keys_app() {
    use crate::defs::*;
    use crate::elements::{Border, Style, StyleDefinition};
    use crate::utils::*;

    use crate::{ItemSelector, ItemSelectorState};
    use embedded_graphics::{
        pixelcolor::{raw::RawU16, Rgb565},
        prelude::Size,
    };

    static PALETTE2: [u16; 4] = [0x0841_u16, 0x4A49_u16, 0xBDF7_u16, 0xFFE7_u16];
    static PALETTE3: [u16; 4] = [0x10A1_u16, 0x56B5_u16, 0xD6F7_u16, 0xFFFF_u16];
    static PALETTE4: [u16; 4] = [0x0842_u16, 0x5295_u16, 0xCE79_u16, 0xFFFF_u16];

    const bordered_style: StyleDefinition = StyleDefinition {
        background: Some(PALETTE_DREAM.darkest),
        margin: EdgeInsets::symmetric(2, 4),
        border: Border::new(PALETTE_DREAM.dark, EdgeInsets::new(1, 2, 3, 4)),
        padding: EdgeInsets::all(2),
    };

    pub fn into565(palette: &[u16; 4], color: u8) -> Rgb565 {
        let raw = palette[(color & 0b11) as usize];
        Rgb565::from(RawU16::new(raw))
    }

    use crate::{
        component::ComponentDefinition, testing_helpers::test_in_window, Element, Stack, Text,
    };

    #[derive(Clone)]
    struct Key {
        text: String,
        key: u32,
    }

    struct AppState {
        keys: Vec<Key>,
        keys_selected_state: crate::ItemSelectorState,
    }

    impl Default for AppState {
        fn default() -> Self {
            Self {
                keys: vec![
                    Key {
                        text: "First".to_string(),
                        key: 1,
                    },
                    Key {
                        text: "Second".to_string(),
                        key: 2,
                    },
                    Key {
                        text: "Third".to_string(),
                        key: 3,
                    },
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
                Style::new_with_style(
                    StyleDefinition {
                        background: if selected {
                            Some(PALETTE_DREAM.light)
                        } else {
                            Some(PALETTE_DREAM.darkest)
                        },
                        ..bordered_style
                    },
                    Some(Text::new(format!("Key: {} {}", key.text, key.key))),
                )
            },
        );

        Stack::col(vec![
            Style::new_with_border(
                Border {
                    color: PALETTE_DREAM.darkest,
                    size: EdgeInsets::all(2),
                },
                Some(Text::new("Main Menu".to_string())),
            ) as Element<AppState>,
            item_selector as Element<AppState>,
        ])
    });

    test_in_window::<AppState>(Size::new(128, 128), main_menu, |_, _| ());
}

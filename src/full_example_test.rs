use embedded_graphics::{
    pixelcolor::{raw::RawU16, Rgb565},
    prelude::Size,
};

use crate::{ItemSelector, ItemSelectorState};

static PALETTE1: [u16; 4] = [0x001F_u16, 0x1CE7_u16, 0x7BEF_u16, 0xFFFF_u16];
static PALETTE2: [u16; 4] = [0x0841_u16, 0x4A49_u16, 0xBDF7_u16, 0xFFE7_u16];
static PALETTE3: [u16; 4] = [0x10A1_u16, 0x56B5_u16, 0xD6F7_u16, 0xFFFF_u16];
static PALETTE4: [u16; 4] = [0x0842_u16, 0x5295_u16, 0xCE79_u16, 0xFFFF_u16];

fn into565(palette: &[u16; 4], color: u8) -> Rgb565 {
    let raw = palette[(color & 0b11) as usize];
    Rgb565::from(RawU16::new(raw))
}

#[ignore]
#[test]
fn create_keys_app() {
    use crate::{
        example_components::ComponentDefinition, testing_helpers::test_in_window, Border, Element,
        Stack, Text,
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
            |_key: &Key, selected: bool| {
                crate::Box::exactly(
                    Size::new(128, 16),
                    into565(&PALETTE2, if selected { 2 } else { 1 }),
                    Some(Text::new("lol")),
                )
            },
        );

        Stack::col(vec![
            Border::bottom(
                1,
                into565(&PALETTE3, 0),
                crate::Box::exactly(
                    Size::new(128, 16),
                    into565(&PALETTE4, state.keys_selected_state.active as u8),
                    Some(Text::new("Main Menu")),
                ),
            ) as Element<AppState>,
            item_selector as Element<AppState>,
        ])
    });

    test_in_window(Size::new(128, 128), main_menu, |_, _| ());
}

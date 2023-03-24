use crate::Runner;

use super::{App, Component, Number, Stack, Text};

#[test]
fn create_leaf_app() {
    let comp: Component<()> = |()| Text::new("Hi");

    let numba: Component<()> = |()| Number::new(333);

    let text_app = App::new(comp, ());

    let numba_app = App::new(numba, ());

    assert_eq!("Hi".to_string(), text_app.to_string());

    assert_eq!("333".to_string(), numba_app.to_string());
}

#[test]
fn create_multichild_app() {
    let comp: Component<()> = |()| Stack::col(vec![Text::new("Hi"), Number::new(333)]);

    let app = App::new(comp, ());

    assert_eq!("[Hi, 333]".to_string(), app.to_string());
}

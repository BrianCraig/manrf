use crate::Runner;

use super::{Component, Text, App, Number};

#[test]
fn create_app() {
    
    let comp:Component<()> = |()| {
        Text::new("Hi")
    };
    
    let numba:Component<()> = |()| {
        Number::new(333)
    };
    
    
    let text_app = App::new(comp, ());
    
    let numba_app = App::new(numba, ());
    
    assert_eq!("Hi".to_string(), text_app.to_string());
    
    assert_eq!("333".to_string(), numba_app.to_string());
}
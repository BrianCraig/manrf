pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
pub enum Button {
    Principal, 
    Secondary,
    Back,
}

pub enum Event {
    DirectionPressed(Direction),
    ButtonPressed(Button)
}


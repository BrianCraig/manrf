#[cfg(test)]
pub mod simulator;

#[derive(Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug)]
pub enum Button {
    Principal,
    Secondary,
    Back,
}


#[derive(Clone, Debug)]
pub enum Event {
    DirectionPressed(Direction),
    ButtonPressed(Button),
}


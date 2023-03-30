use embedded_graphics::prelude::Size;

pub struct Constraints {
    min: Size,
    max: Size,
}

impl Constraints {
    pub fn up_to(size: Size) -> Self {
        Constraints {
            min: Size {
                width: 0,
                height: 0,
            },
            max: size,
        }
    }
}

use embedded_graphics::prelude::Size;

#[derive(Copy, Clone)]
pub struct Constraints {
    pub min: Size,
    pub max: Size,
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

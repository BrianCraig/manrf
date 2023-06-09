use embedded_graphics::prelude::Size;

#[derive(Copy, Clone)]
pub struct Constraints {
    pub min: Size,
    pub max: Size,
}

impl Constraints {
    pub const fn up_to(size: Size) -> Self {
        Constraints {
            min: Size {
                width: 0,
                height: 0,
            },
            max: size,
        }
    }

    pub const fn shrink(self, size: &Size) -> Self {
        Constraints {
            min: Size {
                width: self.min.width.saturating_add(size.width),
                height: self.min.height.saturating_add(size.height),
            },
            max: self.max,
        }
    }

    pub fn clamp(&self, size: &Size) -> Size {
        Size {
            width: size.width.max(self.min.width).min(self.max.width),
            height: size.height.max(self.min.height).min(self.max.height),
        }
    }
}

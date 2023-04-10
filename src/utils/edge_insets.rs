use embedded_graphics::prelude::{Size, Point};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct EdgeInsets {
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub left: u32,
}

impl EdgeInsets {
    pub const fn new(top: u32, right: u32, bottom: u32, left: u32) -> Self {
        EdgeInsets {
            top,
            bottom,
            left,
            right,
        }
    }

    pub const fn all(size: u32) -> Self {
        EdgeInsets {
            top: size,
            bottom: size,
            left: size,
            right: size,
        }
    }

    pub const fn symmetric(vertical: u32, horizontal: u32) -> Self {
        EdgeInsets {
            top: vertical,
            bottom: vertical,
            left: horizontal,
            right: horizontal,
        }
    }

    pub const fn from_top(top: u32) -> Self {
        EdgeInsets {
            top,
            bottom: 0,
            left: 0,
            right: 0,
        }
    }

    pub const fn from_bottom(bottom: u32) -> Self {
        EdgeInsets {
            top: 0,
            bottom,
            left: 0,
            right: 0,
        }
    }

    pub const fn from_left(left: u32) -> Self {
        EdgeInsets {
            top: 0,
            bottom: 0,
            left,
            right: 0,
        }
    }

    pub const fn from_right(right: u32) -> Self {
        EdgeInsets {
            top: 0,
            bottom: 0,
            left: 0,
            right,
        }
    }

    pub const fn height(&self) -> u32 {
        self.top + self.bottom
    }

    pub const fn width(&self) -> u32 {
        self.left + self.right
    }

    pub const fn size(&self) -> Size {
        Size::new(self.width(), self.height())
    }

    pub const fn is_empty(&self) -> bool {
        self.top == 0 && self.bottom == 0 && self.left == 0 && self.right == 0
    }

    pub const fn top_left_offset(&self) -> Point {
        Point::new(self.left as i32, self.top as i32)
    }
    
}
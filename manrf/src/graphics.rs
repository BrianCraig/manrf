use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb888,
    prelude::{DrawTarget, Point},
    primitives::Rectangle,
    text::Text,
    Drawable,
};
pub enum GraphicOperation {
    DrawRectangle {
        rect: Rectangle,
        color: Rgb888,
    },
    DrawText {
        position: Point,
        text: String,
        color: Rgb888,
    },
}
pub struct GraphicOperationQueue {
    operations: Vec<GraphicOperation>,
}

impl GraphicOperationQueue {
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }

    pub fn push(&mut self, operation: GraphicOperation) {
        self.operations.push(operation);
    }

    pub fn drain(&mut self) -> impl Iterator<Item = GraphicOperation> + '_ {
        self.operations.drain(..)
    }
}

pub trait GraphicsEndpoint {
    fn draw_queue(&mut self, queue: GraphicOperationQueue);
}

pub struct EmbeddedGraphicsEndpoint<T: DrawTarget<Color = Rgb888>>
{
    pub target: T,
}

impl<T: DrawTarget<Color = Rgb888>> EmbeddedGraphicsEndpoint<T> {
    pub fn new(target: T) -> Self {
        Self { target }
    }
}

impl<T> GraphicsEndpoint for EmbeddedGraphicsEndpoint<T>
where
    T: DrawTarget<Color = Rgb888>,
{
    fn draw_queue(&mut self, mut queue: GraphicOperationQueue) {
        for operation in queue.drain() {
            match operation {
                GraphicOperation::DrawRectangle { rect, color } => {
                    let _ = self.target.fill_solid(&rect, color);
                }
                GraphicOperation::DrawText {
                    position,
                    text,
                    color,
                } => {
                    let _ = Text::new(&text, position + Point::new(0, FONT_6X10.baseline as i32), MonoTextStyle::new(&FONT_6X10, color))
                        .draw(&mut self.target);
                }
            }
        }
    }
}

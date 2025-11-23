use img::rgba::Rgba;

use crate::Canvas;

pub struct StrokeRect {
    canvas: Canvas,
    thickness: u32,
}

impl StrokeRect {
    pub fn new(canvas: Canvas, thickness: u32) -> Self {
        Self { canvas, thickness }
    }

    pub fn fill(&self, color: Rgba) {
        self.canvas.draw_filled_rect(x, y, width, height, color);
    }
}

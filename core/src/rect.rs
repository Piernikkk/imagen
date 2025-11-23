use img::rgba::Rgba;

use crate::Canvas;

pub struct StrokeRect<'a> {
    canvas: &'a mut Canvas,
    thickness: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl<'a> StrokeRect<'a> {
    pub fn new(
        canvas: &'a mut Canvas,
        thickness: u32,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) -> Self {
        Self {
            canvas,
            thickness,
            x,
            y,
            width,
            height,
        }
    }

    pub fn fill(&mut self, color: Rgba) {
        self.canvas.draw_filled_rect(
            self.x + self.thickness,
            self.y + self.thickness,
            self.width - 2 * self.thickness,
            self.height - 2 * self.thickness,
            color,
        );
    }
}

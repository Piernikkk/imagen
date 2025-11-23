use crate::Canvas;
use img::rgba::Rgba;

pub struct StrokeCircle<'a> {
    canvas: &'a mut Canvas,
    cx: u32,
    cy: u32,
    radius: u32,
    thickness: u32,
}

impl<'a> StrokeCircle<'a> {
    pub fn new(canvas: &'a mut Canvas, cx: u32, cy: u32, radius: u32, thickness: u32) -> Self {
        Self {
            canvas,
            cx,
            cy,
            radius,
            thickness,
        }
    }

    pub fn fill(&mut self, color: Rgba) {
        self.canvas
            .draw_filled_circle(self.cx, self.cy, self.radius - self.thickness, color);
    }
}

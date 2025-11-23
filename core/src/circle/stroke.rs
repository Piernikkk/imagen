use color_eyre::eyre::{Result, eyre};
use img::rgba::Rgba;

use crate::Canvas;

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

    pub fn fill(&mut self, color: Rgba) -> &mut Self {
        self.canvas
            .draw_filled_circle(self.cx, self.cy, self.radius - self.thickness, color);

        self
    }

    pub fn to_canvas(&mut self) -> &mut Canvas {
        self.canvas
    }
}

impl Canvas {
    pub fn draw_stroke_circle(
        &mut self,
        cx: u32,
        cy: u32,
        radius: u32,
        thickness: u32,
        color: Rgba,
    ) -> Result<StrokeCircle> {
        if thickness > radius {
            return Err(eyre!("Stroke thickness can't be bigger than than radius"));
        }

        let outer_r_sq = (radius * radius) as i32;
        let inner_radius = radius.saturating_sub(thickness);
        let inner_r_sq = (inner_radius * inner_radius) as i32;

        for dy in -(radius as i32)..=(radius as i32) {
            for dx in -(radius as i32)..=(radius as i32) {
                let dist_sq = dx * dx + dy * dy;
                if dist_sq <= outer_r_sq && dist_sq >= inner_r_sq {
                    let px = (cx as i32 + dx) as u32;
                    let py = (cy as i32 + dy) as u32;

                    if px < self.image.width && py < self.image.height {
                        self.image.set_pixel(px, py, color);
                    }
                }
            }
        }

        Ok(StrokeCircle::new(self, cx, cy, radius, thickness))
    }
}

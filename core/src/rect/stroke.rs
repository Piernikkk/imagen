use crate::Canvas;

use color_eyre::eyre::{Result, eyre};
use img::rgba::Rgba;

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

    pub fn to_canvas(&mut self) -> &mut Canvas {
        self.canvas
    }
}

impl Canvas {
    pub fn draw_stroke_rect(
        &mut self,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        thickness: u32,
        color: Rgba,
    ) -> Result<StrokeRect> {
        if thickness > y + height || thickness > x + width {
            return Err(eyre!("Thinkness can't be bigger than the rect itself"));
        }

        self.draw_filled_rect(x, y, width, thickness, color);
        self.draw_filled_rect(x, y + height - thickness, width, thickness, color);
        self.draw_filled_rect(x, y, thickness, height, color);
        self.draw_filled_rect(x + width - thickness, y, thickness, height, color);

        Ok(StrokeRect::new(self, thickness, x, y, width, height))
    }

    pub fn draw_rounded_stroke_rect(
        &mut self,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        thickness: u32,
        radius: u32,
        color: Rgba,
    ) -> &mut Self {
        let radius = radius.min(width / 2).min(height / 2);

        if radius * 2 < width {
            self.draw_filled_rect(x + radius, y, width - 2 * radius, thickness, color);
            self.draw_filled_rect(
                x + radius,
                y + height - thickness,
                width - 2 * radius,
                thickness,
                color,
            );
        }

        if radius * 2 < height {
            self.draw_filled_rect(x, y + radius, thickness, height - 2 * radius, color);
            self.draw_filled_rect(
                x + width - thickness,
                y + radius,
                thickness,
                height - 2 * radius,
                color,
            );
        }

        self.draw_stroke_arc(x + radius, y + radius, radius, thickness, color, 180, 270);
        self.draw_stroke_arc(
            x + width - radius - 1,
            y + radius,
            radius,
            thickness,
            color,
            270,
            360,
        );
        self.draw_stroke_arc(
            x + radius,
            y + height - radius - 1,
            radius,
            thickness,
            color,
            90,
            180,
        );
        self.draw_stroke_arc(
            x + width - radius - 1,
            y + height - radius - 1,
            radius,
            thickness,
            color,
            0,
            90,
        );

        self
    }
}

pub mod circle;
pub mod rect;

use color_eyre::eyre::{Ok, Result, eyre};
use std::fs;
use std::path::{Path, PathBuf};

pub use img::{
    codecs::{Codecs, save_png},
    rgba::Rgba,
    rgba_image::RgbaImage,
};

use crate::circle::StrokeCircle;
use crate::rect::StrokeRect;

#[derive(Debug, Clone)]
pub struct Canvas {
    pub image: RgbaImage,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            image: RgbaImage::new(width, height),
        }
    }

    pub fn save(&self, path: PathBuf, codec: Codecs) -> Result<()> {
        let full_path = Path::new("output").join(path);

        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)?;
        }

        match codec {
            Codecs::PNG => save_png(full_path, &self.image),
        }
    }

    pub fn draw_pixel(&mut self, x: u32, y: u32, color: Rgba) -> Result<&mut Self> {
        if x >= self.image.width || y >= self.image.height {
            return Err(eyre!("X or Y is not in image bounds"));
        }

        self.image.set_pixel(x, y, color);

        Ok(self)
    }

    pub fn draw_filled_rect(
        &mut self,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        color: Rgba,
    ) -> &mut Self {
        let x_end = (x + width).min(self.image.width);
        let y_end = (y + height).min(self.image.height);

        for py in y..y_end {
            for px in x..x_end {
                self.image.set_pixel(px, py, color);
            }
        }
        self
    }

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

    pub fn draw_rounded_filled_rect(
        &mut self,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        radius: u32,
        color: Rgba,
    ) -> &mut Self {
        let radius = radius.min(width / 2).min(height / 2);

        self.draw_filled_rect(x + radius, y, width - 2 * radius, height, color);

        self.draw_filled_rect(x, y + radius, radius, height - 2 * radius, color);
        self.draw_filled_rect(
            x + width - radius,
            y + radius,
            radius,
            height - 2 * radius,
            color,
        );

        self.draw_filled_arc(x + radius, y + radius, radius, color, 180, 270);
        self.draw_filled_arc(x + width - radius - 1, y + radius, radius, color, 270, 360);
        self.draw_filled_arc(x + radius, y + height - radius - 1, radius, color, 90, 180);
        self.draw_filled_arc(
            x + width - radius - 1,
            y + height - radius - 1,
            radius,
            color,
            0,
            90,
        );

        self
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

    pub fn draw_filled_circle(&mut self, cx: u32, cy: u32, radius: u32, color: Rgba) -> &mut Self {
        let r_sq = (radius * radius) as i32;

        for dy in -(radius as i32)..=(radius as i32) {
            for dx in -(radius as i32)..=(radius as i32) {
                if dx * dx + dy * dy <= r_sq {
                    let px = (cx as i32 + dx) as u32;
                    let py = (cy as i32 + dy) as u32;

                    if px < self.image.width && py < self.image.height {
                        self.image.set_pixel(px, py, color);
                    }
                }
            }
        }
        self
    }

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

    fn draw_filled_arc(
        &mut self,
        cx: u32,
        cy: u32,
        radius: u32,
        color: Rgba,
        start_angle: u32,
        end_angle: u32,
    ) -> &mut Self {
        let r_sq = (radius * radius) as i32;

        for dy in -(radius as i32)..=(radius as i32) {
            for dx in -(radius as i32)..=(radius as i32) {
                let dist_sq = dx * dx + dy * dy;
                if dist_sq <= r_sq {
                    let angle = ((dy as f64).atan2(dx as f64).to_degrees() + 360.0) % 360.0;

                    let in_range = if start_angle <= end_angle {
                        angle >= start_angle as f64 && angle <= end_angle as f64
                    } else {
                        angle >= start_angle as f64 || angle <= end_angle as f64
                    };

                    if in_range {
                        let px = (cx as i32 + dx) as u32;
                        let py = (cy as i32 + dy) as u32;

                        if px < self.image.width && py < self.image.height {
                            self.image.set_pixel(px, py, color);
                        }
                    }
                }
            }
        }

        self
    }

    fn draw_stroke_arc(
        &mut self,
        cx: u32,
        cy: u32,
        radius: u32,
        thickness: u32,
        color: Rgba,
        start_angle: u32,
        end_angle: u32,
    ) -> &mut Self {
        let outer_r_sq = (radius * radius) as i32;
        let inner_radius = radius.saturating_sub(thickness);
        let inner_r_sq = (inner_radius * inner_radius) as i32;

        for dy in -(radius as i32)..=(radius as i32) {
            for dx in -(radius as i32)..=(radius as i32) {
                let dist_sq = dx * dx + dy * dy;
                if dist_sq <= outer_r_sq && dist_sq >= inner_r_sq {
                    let angle = ((dy as f64).atan2(dx as f64).to_degrees() + 360.0) % 360.0;

                    let in_range = if start_angle <= end_angle {
                        angle >= start_angle as f64 && angle <= end_angle as f64
                    } else {
                        angle >= start_angle as f64 || angle <= end_angle as f64
                    };

                    if in_range {
                        let px = (cx as i32 + dx) as u32;
                        let py = (cy as i32 + dy) as u32;

                        if px < self.image.width && py < self.image.height {
                            self.image.set_pixel(px, py, color);
                        }
                    }
                }
            }
        }

        self
    }
}

pub mod rect;

use color_eyre::eyre::{Result, eyre};
use std::fs;
use std::path::{Path, PathBuf};

pub use img::{
    codecs::{Codecs, save_png},
    rgba::Rgba,
    rgba_image::RgbaImage,
};

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

    pub fn draw_pixel(&mut self, x: u32, y: u32, color: Rgba) -> Result<()> {
        if x >= self.image.width || y >= self.image.height {
            return Err(eyre!("X or Y is not in image bounds"));
        }

        self.image.set_pixel(x, y, color);

        Ok(())
    }

    pub fn draw_filled_rect(&mut self, x: u32, y: u32, width: u32, height: u32, color: Rgba) {
        let x_end = (x + width).min(self.image.width);
        let y_end = (y + height).min(self.image.height);

        for py in y..y_end {
            for px in x..x_end {
                self.image.set_pixel(px, py, color);
            }
        }
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
            return Err(eyre!("Thinkness is bigger than rect itself"));
        }

        self.draw_filled_rect(x, y, width, thickness, color);
        self.draw_filled_rect(x, y + height - thickness, width, thickness, color);
        self.draw_filled_rect(x, y, thickness, height, color);
        self.draw_filled_rect(x + width - thickness, y, thickness, height, color);

        Ok(StrokeRect::new(self.clone(), thickness))
    }
}

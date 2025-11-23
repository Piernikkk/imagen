pub mod arc;
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
}

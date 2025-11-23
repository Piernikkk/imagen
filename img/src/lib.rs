use crate::{rgba::Rgba, rgba_image::RgbaImage};
use color_eyre::eyre::Result;

pub mod codecs;
pub mod rgba;
pub mod rgba_image;

pub struct Canvas {
    pub image: RgbaImage,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            image: RgbaImage::new(width, height),
        }
    }

    pub fn draw_pixel(&mut self, x: u32, y: u32, color: Rgba) -> Result<()> {
        self.image.set_pixel(x, y, color)
    }

    // pub fn save(&self, path: &str) -> Result<()> {
    //     let
    // }
}

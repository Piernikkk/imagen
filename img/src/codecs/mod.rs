pub mod png;

use std::{fs::write, path::PathBuf};

use color_eyre::eyre::{Result, eyre};

use crate::{codecs::png::encode_to_png, rgba_image::RgbaImage};

pub enum Codecs {
    PNG,
}

pub fn save_png(path: PathBuf, image: &RgbaImage) -> Result<()> {
    let png_data = encode_to_png(image)?;

    write(path.clone(), png_data)
        .map_err(|e| eyre!("Failed to write PNG file to {:?}: {}", path, e))?;

    Ok(())
}

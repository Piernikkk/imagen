use std::path::Path;

use color_eyre::eyre::{Ok, Result};
use imagen::{Canvas, Rgba};

fn main() -> Result<()> {
    color_eyre::install()?;

    println!("Hello, world!");

    generate_test_image()?;

    Ok(())
}

fn generate_test_image() -> Result<()> {
    let mut canvas = Canvas::new(20, 20);

    canvas.draw_pixel(
        10,
        10,
        Rgba {
            r: 100,
            g: 100,
            b: 100,
            a: 255,
        },
    )?;

    canvas.save(Path::new("test.png").to_path_buf(), imagen::Codecs::PNG)?;
    Ok(())
}

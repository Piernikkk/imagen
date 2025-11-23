use std::path::Path;

use color_eyre::eyre::{Ok, Result};
use imagen::{Canvas, Rgba};

fn main() -> Result<()> {
    color_eyre::install()?;

    println!("Hello, world!");

    generate_test_image()?;
    generate_rects()?;
    generate_stroke_rect()?;

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

fn generate_rects() -> Result<()> {
    let mut canvas = Canvas::new(100, 100);

    canvas.draw_filled_rect(20, 10, 50, 50, (200, 200, 30).into());

    canvas.save(Path::new("rect.png").to_path_buf(), imagen::Codecs::PNG)?;

    Ok(())
}

fn generate_stroke_rect() -> Result<()> {
    let mut canvas = Canvas::new(100, 100);

    canvas.draw_stroke_rect(20, 10, 50, 50, 10, (200, 200, 30).into())?;

    canvas.save(
        Path::new("stroke_rect.png").to_path_buf(),
        imagen::Codecs::PNG,
    )?;

    Ok(())
}

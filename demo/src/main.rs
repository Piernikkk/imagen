use std::path::Path;

use color_eyre::eyre::{Ok, Result};
use imagen::{Canvas, Rgba};

const FONT: &[u8] = include_bytes!("../fonts/Poppins-Regular.ttf");

fn main() -> Result<()> {
    color_eyre::install()?;

    println!("Hello, world!");

    generate_test_image()?;
    generate_rect()?;
    generate_stroke_rect()?;
    generate_filled_rect()?;
    generate_rounded_rect()?;
    generate_rounded_stroke_rect()?;
    generate_circle()?;
    generate_stroke_circle()?;
    generate_filled_stroke_circle()?;

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

fn generate_rect() -> Result<()> {
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

fn generate_filled_rect() -> Result<()> {
    let mut canvas = Canvas::new(100, 100);

    canvas
        .draw_stroke_rect(20, 10, 50, 50, 10, (200, 200, 30).into())?
        .fill((200, 30, 200).into());

    canvas.save(
        Path::new("filled_rect.png").to_path_buf(),
        imagen::Codecs::PNG,
    )?;

    Ok(())
}

fn generate_rounded_rect() -> Result<()> {
    let mut canvas = Canvas::new(200, 200);

    canvas.draw_rounded_filled_rect(10, 10, 150, 150, 30, (200, 200, 30).into());

    canvas.save(
        Path::new("rounded_rect.png").to_path_buf(),
        imagen::Codecs::PNG,
    )?;

    Ok(())
}

fn generate_rounded_stroke_rect() -> Result<()> {
    let mut canvas = Canvas::new(200, 200);

    canvas.draw_rounded_stroke_rect(10, 10, 150, 150, 20, 30, (200, 200, 30).into());

    canvas.save(
        Path::new("rounded_stroke_rect.png").to_path_buf(),
        imagen::Codecs::PNG,
    )?;

    Ok(())
}

fn generate_circle() -> Result<()> {
    let mut canvas = Canvas::new(200, 200);

    canvas
        .draw_filled_circle(150, 100, 80, (200, 200, 30).into())
        .draw_rounded_stroke_rect(10, 10, 30, 30, 5, 5, (200, 200, 30).into())
        .draw_text("TEST", 50, 50, FONT, 96.0, (255, 255, 255).into())?;

    canvas.save(Path::new("circle.png").to_path_buf(), imagen::Codecs::PNG)?;

    Ok(())
}

fn generate_stroke_circle() -> Result<()> {
    let mut canvas = Canvas::new(200, 200);

    canvas.draw_stroke_circle(150, 100, 80, 10, (200, 200, 30).into())?;

    canvas.save(
        Path::new("stroke_circle.png").to_path_buf(),
        imagen::Codecs::PNG,
    )?;

    Ok(())
}

fn generate_filled_stroke_circle() -> Result<()> {
    let mut canvas = Canvas::new(200, 200);

    canvas
        .draw_stroke_circle(150, 100, 80, 10, (200, 200, 30).into())?
        .fill((0, 255, 0).into())
        .to_canvas()
        .draw_text("testasfdasfasf", 50, 50, FONT, 48.0, (0, 0, 0).into())?;

    canvas.save(
        Path::new("filled_circle.png").to_path_buf(),
        imagen::Codecs::PNG,
    )?;

    Ok(())
}

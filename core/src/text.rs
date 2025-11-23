use ab_glyph::{Font, FontRef, PxScale, ScaleFont};
use color_eyre::eyre::{Result, eyre};
use img::rgba::Rgba;

use crate::Canvas;

impl Canvas {
    pub fn draw_text(
        &mut self,
        text: &str,
        x: u32,
        y: u32,
        font_data: &[u8],
        font_size: f32,
        color: Rgba,
    ) -> Result<&mut Self> {
        let font = FontRef::try_from_slice(font_data).map_err(|e| eyre!("Font is invalid: {e}"))?;

        let scale = PxScale::from(font_size);
        let scaled_font = font.as_scaled(scale);

        let mut cursor_x = x as f32;
        let ascent = scaled_font.ascent();

        for char in text.chars() {
            let glyph = scaled_font.scaled_glyph(char);
            let outlined = scaled_font.outline_glyph(glyph.clone());

            if let Some(outlined) = outlined {
                let bounds = outlined.px_bounds();

                outlined.draw(|px, py, coverage| {
                    if coverage > 0.0 {
                        let pixel_x = (cursor_x + px as f32 + bounds.min.x) as i32;
                        let pixel_y = (y as f32 + py as f32 + bounds.min.y + ascent) as i32;

                        if pixel_x >= 0
                            && pixel_y >= 0
                            && (pixel_x as u32) < self.image.width
                            && (pixel_y as u32) < self.image.height
                        {
                            let a = (coverage * color.a as f32) as u8;
                            let text_color = Rgba { a, ..color };

                            let existing = self.image.get_pixel(pixel_x as u32, pixel_y as u32);
                            let blended = blend_colors(existing, text_color);
                            self.image
                                .set_pixel(pixel_x as u32, pixel_y as u32, blended);
                        }
                    }
                });
            };

            cursor_x += scaled_font.h_advance(glyph.id);
        }

        Ok(self)
    }
}

fn blend_colors(bg: Rgba, fg: Rgba) -> Rgba {
    if bg.a == 0 {
        return fg;
    }

    if fg.a == 0 {
        return bg;
    }

    let fg_alpha = fg.a as f32 / 255.0;
    let bg_alpha = bg.a as f32 / 255.0;

    let alpha = fg_alpha + bg_alpha * (1.0 - fg_alpha);

    if alpha == 0.0 {
        return Rgba {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        };
    }

    Rgba {
        r: ((fg.r as f32 * fg_alpha + bg.r as f32 * bg_alpha * (1.0 - fg_alpha)) / alpha) as u8,
        g: ((fg.g as f32 * fg_alpha + bg.g as f32 * bg_alpha * (1.0 - fg_alpha)) / alpha) as u8,
        b: ((fg.b as f32 * fg_alpha + bg.b as f32 * bg_alpha * (1.0 - fg_alpha)) / alpha) as u8,
        a: (alpha * 255.0) as u8,
    }
}

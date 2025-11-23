use img::rgba::Rgba;

use crate::Canvas;

impl Canvas {
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
}

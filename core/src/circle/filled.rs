use img::rgba::Rgba;

use crate::Canvas;

impl Canvas {
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
}

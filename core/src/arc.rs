use img::rgba::Rgba;

use crate::Canvas;

impl Canvas {
    pub fn draw_filled_arc(
        &mut self,
        cx: u32,
        cy: u32,
        radius: u32,
        color: Rgba,
        start_angle: u32,
        end_angle: u32,
    ) -> &mut Self {
        let r_sq = (radius * radius) as i32;

        for dy in -(radius as i32)..=(radius as i32) {
            for dx in -(radius as i32)..=(radius as i32) {
                let dist_sq = dx * dx + dy * dy;
                if dist_sq <= r_sq {
                    let angle = ((dy as f64).atan2(dx as f64).to_degrees() + 360.0) % 360.0;

                    let in_range = if start_angle <= end_angle {
                        angle >= start_angle as f64 && angle <= end_angle as f64
                    } else {
                        angle >= start_angle as f64 || angle <= end_angle as f64
                    };

                    if in_range {
                        let px = (cx as i32 + dx) as u32;
                        let py = (cy as i32 + dy) as u32;

                        if px < self.image.width && py < self.image.height {
                            self.image.set_pixel(px, py, color);
                        }
                    }
                }
            }
        }

        self
    }

    pub fn draw_stroke_arc(
        &mut self,
        cx: u32,
        cy: u32,
        radius: u32,
        thickness: u32,
        color: Rgba,
        start_angle: u32,
        end_angle: u32,
    ) -> &mut Self {
        let outer_r_sq = (radius * radius) as i32;
        let inner_radius = radius.saturating_sub(thickness);
        let inner_r_sq = (inner_radius * inner_radius) as i32;

        for dy in -(radius as i32)..=(radius as i32) {
            for dx in -(radius as i32)..=(radius as i32) {
                let dist_sq = dx * dx + dy * dy;
                if dist_sq <= outer_r_sq && dist_sq >= inner_r_sq {
                    let angle = ((dy as f64).atan2(dx as f64).to_degrees() + 360.0) % 360.0;

                    let in_range = if start_angle <= end_angle {
                        angle >= start_angle as f64 && angle <= end_angle as f64
                    } else {
                        angle >= start_angle as f64 || angle <= end_angle as f64
                    };

                    if in_range {
                        let px = (cx as i32 + dx) as u32;
                        let py = (cy as i32 + dy) as u32;

                        if px < self.image.width && py < self.image.height {
                            self.image.set_pixel(px, py, color);
                        }
                    }
                }
            }
        }

        self
    }
}

use crate::rgba::Rgba;

#[derive(Debug, Clone)]
pub struct RgbaImage {
    pub width: u32,
    pub height: u32,
    pub data: Vec<Rgba>,
}

impl RgbaImage {
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        Self {
            width,
            height,
            data: vec![Rgba::default(); size],
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Rgba) {
        let index = (y * self.width + x) as usize;
        self.data[index] = color;
    }

    pub fn get_pixel(&mut self, x: u32, y: u32) -> Rgba {
        let index = (y * self.width + x) as usize;
        self.data[index]
    }

    pub fn to_u8_vec(&self) -> Vec<u8> {
        self.data.iter().flat_map(|d| d.to_array()).collect()
    }
}

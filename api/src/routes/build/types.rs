use imagen::Rgba;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
#[schema(title = "DrawCommand")]
pub enum DrawCommand {
    /// Draw a filled rectangle
    #[schema(title = "FilledRect")]
    FilledRect {
        /// X coordinate of the top-left corner
        x: u32,
        /// Y coordinate of the top-left corner
        y: u32,
        /// Width of the rectangle
        width: u32,
        /// Height of the rectangle
        height: u32,
        /// Fill color
        color: Color,
    },
    /// Draw a rectangle outline
    #[schema(title = "StrokeRect")]
    StrokeRect {
        /// X coordinate of the top-left corner
        x: u32,
        /// Y coordinate of the top-left corner
        y: u32,
        /// Width of the rectangle
        width: u32,
        /// Height of the rectangle
        height: u32,
        /// Thickness of the outline
        thickness: u32,
        /// Outline color
        color: Color,
    },
    /// Draw a filled rectangle with rounded corners
    #[schema(title = "RoundedFilledRect")]
    RoundedFilledRect {
        /// X coordinate of the top-left corner
        x: u32,
        /// Y coordinate of the top-left corner
        y: u32,
        /// Width of the rectangle
        width: u32,
        /// Height of the rectangle
        height: u32,
        /// Corner radius
        radius: u32,
        /// Fill color
        color: Color,
    },
    /// Draw a rounded rectangle outline
    #[schema(title = "RoundedStrokeRect")]
    RoundedStrokeRect {
        /// X coordinate of the top-left corner
        x: u32,
        /// Y coordinate of the top-left corner
        y: u32,
        /// Width of the rectangle
        width: u32,
        /// Height of the rectangle
        height: u32,
        /// Thickness of the outline
        thickness: u32,
        /// Corner radius
        radius: u32,
        /// Outline color
        color: Color,
    },
    /// Draw a filled circle
    #[schema(title = "FilledCircle")]
    FilledCircle {
        /// X coordinate of the center
        cx: u32,
        /// Y coordinate of the center
        cy: u32,
        /// Radius of the circle
        radius: u32,
        /// Fill color
        color: Color,
    },
    /// Draw a circle outline
    #[schema(title = "StrokeCircle")]
    StrokeCircle {
        /// X coordinate of the center
        cx: u32,
        /// Y coordinate of the center
        cy: u32,
        /// Radius of the circle
        radius: u32,
        /// Thickness of the outline
        thickness: u32,
        /// Outline color
        color: Color,
    },
    /// Draw text
    #[schema(title = "Text")]
    Text {
        /// Text content to render
        text: String,
        /// X coordinate of the text baseline
        x: u32,
        /// Y coordinate of the text baseline
        y: u32,
        /// Font size in pixels
        font_size: f32,
        /// Text color
        color: Color,
        /// Optional font name (currently unused, defaults to Roboto-Regular)
        #[serde(default)]
        font: Option<String>,
    },
    /// Draw a single pixel
    #[schema(title = "Pixel")]
    Pixel {
        /// X coordinate
        x: u32,
        /// Y coordinate
        y: u32,
        /// Pixel color
        color: Color,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[schema(title = "Color", description = "RGBA color with values from 0-255")]
pub struct Color {
    /// Red channel (0-255)
    #[schema(minimum = 0, maximum = 255)]
    pub r: u8,
    /// Green channel (0-255)
    #[schema(minimum = 0, maximum = 255)]
    pub g: u8,
    /// Blue channel (0-255)
    #[schema(minimum = 0, maximum = 255)]
    pub b: u8,
    /// Alpha channel (0-255, where 255 is fully opaque)
    #[schema(minimum = 0, maximum = 255)]
    pub a: u8,
}

impl From<Color> for Rgba {
    fn from(color: Color) -> Self {
        Rgba {
            r: color.r,
            g: color.g,
            b: color.b,
            a: color.a,
        }
    }
}

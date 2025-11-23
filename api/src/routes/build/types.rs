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

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CommandTypeInfo {
    /// The command type identifier
    pub value: String,
    /// Human-readable label for the command
    pub label: String,
    /// Description of what this command does
    pub description: String,
    /// List of required fields for this command
    pub fields: Vec<String>,
}

impl DrawCommand {
    /// Get metadata for all available command types
    pub fn get_all_types() -> Vec<CommandTypeInfo> {
        use crate::command_types;
        use crate::macros::{to_snake_case, to_title_case};

        command_types! {
            FilledRect => "Draw a filled rectangle" { x, y, width, height, color },
            StrokeRect => "Draw a rectangle outline" { x, y, width, height, thickness, color },
            RoundedFilledRect => "Draw a filled rectangle with rounded corners" { x, y, width, height, radius, color },
            RoundedStrokeRect => "Draw a rounded rectangle outline" { x, y, width, height, thickness, radius, color },
            FilledCircle => "Draw a filled circle" { cx, cy, radius, color },
            StrokeCircle => "Draw a circle outline" { cx, cy, radius, thickness, color },
            Text => "Draw text" { text, x, y, font_size, color },
            Pixel => "Draw a single pixel" { x, y, color }
        }
    }
}

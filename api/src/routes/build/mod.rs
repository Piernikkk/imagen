pub mod types;

use axum::{
    Json,
    http::{StatusCode, header},
    response::IntoResponse,
};
use color_eyre::eyre::eyre;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use imagen::Canvas;

use crate::{
    axum_error::{AxumError, AxumResult},
    routes::build::types::{Color, DrawCommand},
    state::AppState,
};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(generate_image))
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ImageRequest {
    pub width: u32,
    pub height: u32,
    #[serde(default)]
    pub background: Option<Color>,
    pub commands: Vec<DrawCommand>,
}

/// Draws an image based on drawing commands
///
/// Creates an image with the specified dimensions and applies a series of drawing commands in order.
/// Returns the generated image as a PNG.
#[utoipa::path(
    post,
    path = "",
    request_body = ImageRequest,
    responses(
        (status = 200, description = "Image generated successfully", content_type = "image/png"),
        (status = 400, description = "Invalid request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "imagen"
)]
async fn generate_image(Json(request): Json<ImageRequest>) -> AxumResult<impl IntoResponse> {
    if request.width == 0 || request.height == 0 {
        return Err(AxumError::bad_request(eyre!(
            "Width and height must be greater than 0"
        )));
    }

    if request.width > 4096 || request.height > 4096 {
        return Err(AxumError::bad_request(eyre!(
            "Width and height must not exceed 4096 pixels"
        )));
    }

    let mut canvas = Canvas::new(request.width, request.height);

    if let Some(bg) = request.background {
        canvas.draw_filled_rect(0, 0, request.width, request.height, bg.into());
    }

    const DEFAULT_FONT: &[u8] = include_bytes!("../../../fonts/Roboto-Regular.ttf");

    for command in request.commands {
        match command {
            DrawCommand::FilledRect {
                x,
                y,
                width,
                height,
                color,
            } => {
                canvas.draw_filled_rect(x, y, width, height, color.into());
            }
            DrawCommand::StrokeRect {
                x,
                y,
                width,
                height,
                thickness,
                color,
            } => {
                canvas
                    .draw_stroke_rect(x, y, width, height, thickness, color.into())
                    .map_err(|e| {
                        AxumError::bad_request(eyre!("Failed to draw stroke rect: {}", e))
                    })?;
            }
            DrawCommand::RoundedFilledRect {
                x,
                y,
                width,
                height,
                radius,
                color,
            } => {
                canvas.draw_rounded_filled_rect(x, y, width, height, radius, color.into());
            }
            DrawCommand::RoundedStrokeRect {
                x,
                y,
                width,
                height,
                thickness,
                radius,
                color,
            } => {
                canvas.draw_rounded_stroke_rect(
                    x,
                    y,
                    width,
                    height,
                    thickness,
                    radius,
                    color.into(),
                );
            }
            DrawCommand::FilledCircle {
                cx,
                cy,
                radius,
                color,
            } => {
                canvas.draw_filled_circle(cx, cy, radius, color.into());
            }
            DrawCommand::StrokeCircle {
                cx,
                cy,
                radius,
                thickness,
                color,
            } => {
                canvas
                    .draw_stroke_circle(cx, cy, radius, thickness, color.into())
                    .map_err(|e| {
                        AxumError::bad_request(eyre!("Failed to draw stroke circle: {}", e))
                    })?;
            }
            DrawCommand::Text {
                text,
                x,
                y,
                font_size,
                color,
                font: _,
            } => {
                canvas
                    .draw_text(&text, x, y, DEFAULT_FONT, font_size, color.into())
                    .map_err(|e| AxumError::bad_request(eyre!("Failed to draw text: {}", e)))?;
            }
            DrawCommand::Pixel { x, y, color } => {
                canvas
                    .draw_pixel(x, y, color.into())
                    .map_err(|e| AxumError::bad_request(eyre!("Failed to draw pixel: {}", e)))?;
            }
        }
    }

    let png_bytes = canvas_to_png_bytes(&canvas)?;

    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "image/png")],
        png_bytes,
    ))
}

fn canvas_to_png_bytes(canvas: &Canvas) -> AxumResult<Vec<u8>> {
    use std::fs;
    use std::path::PathBuf;

    let temp_path = PathBuf::from(format!(
        "/tmp/imagen_{}.png",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));

    imagen::save_png(temp_path.clone(), &canvas.image)?;

    let bytes =
        fs::read(&temp_path).map_err(|e| AxumError::new(eyre!("Failed to read PNG: {}", e)))?;

    let _ = fs::remove_file(temp_path);

    Ok(bytes)
}

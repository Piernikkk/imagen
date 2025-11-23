use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    routes::build::types::{CommandTypeInfo, DrawCommand},
    state::AppState,
};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(get_command_types))
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CommandTypesResponse {
    pub commands: Vec<CommandTypeInfo>,
}

/// Get available command types
///
/// Returns a list of all available drawing command types with their metadata.
#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "List of available command types", body = CommandTypesResponse)
    ),
    tag = "images"
)]
async fn get_command_types() -> Json<CommandTypesResponse> {
    Json(CommandTypesResponse {
        commands: DrawCommand::get_all_types(),
    })
}

pub mod build;
mod commands;
mod health;

use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

use crate::{ApiDoc, state::AppState};

pub fn routes() -> OpenApiRouter<AppState> {
    let api = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/health", health::routes())
        .nest("/build", build::routes())
        .nest("/commands", commands::routes());

    OpenApiRouter::with_openapi(ApiDoc::openapi()).nest("/api", api)
}

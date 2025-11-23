use axum::{Extension, Json, Router, response::IntoResponse, routing::get};
use color_eyre::eyre::Result;
use http::StatusCode;
use std::net::{Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;
use tracing::level_filters::LevelFilter;
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    fmt::format::FmtSpan, layer::SubscriberExt as _, util::SubscriberInitExt as _,
};
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as _};

use crate::{routes::routes, state::AppState};

pub fn init_tracing() -> Result<()> {
    tracing_subscriber::Registry::default()
        .with(tracing_subscriber::fmt::layer().with_span_events(FmtSpan::NEW | FmtSpan::CLOSE))
        .with(ErrorLayer::default())
        .with(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .with_env_var("RUST_LOG")
                .from_env()?,
        )
        .try_init()?;

    Ok(())
}

pub async fn init_axum(state: AppState) -> Result<Router> {
    let router = routes();

    let (router, api) = router.with_state(state.clone()).split_for_parts();

    let openapi_path = "/docs";
    let spec = "/openapi.json";

    let docs = Router::new()
        .merge(Redoc::with_url("/redoc", api.clone()))
        .merge(RapiDoc::new(format!("{openapi_path}{spec}")).path("/rapidoc"))
        .merge(Scalar::with_url("/scalar", api.clone()))
        .route(spec, get(|| async { Json(api) }));

    let router = router
        .nest(openapi_path, docs)
        .layer(Extension(state))
        .fallback(|| async { (StatusCode::NOT_FOUND, "Site not found").into_response() });

    Ok(router)
}

pub async fn init_listener() -> Result<TcpListener> {
    const DEFAULT_PORT: u16 = 4000;

    let addr: SocketAddr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), DEFAULT_PORT);

    Ok(TcpListener::bind(addr).await?)
}

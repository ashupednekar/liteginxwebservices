use axum::{
    routing::get,
    Router,
};

use crate::prelude::Result;
use super::handlers::probes::{healthz, livez};
use super::state::AppState;


pub async fn build_routes() -> Result<Router> {
    Ok(Router::new()
        .route("/livez/", get(livez))
        .route("/healthz/", get(healthz))
        .with_state(AppState::new().await?))
}

use axum::middleware::from_fn_with_state;
use axum::{Router, routing::get};

use super::handlers::probes::{healthz, livez};
use super::middlewares;
use super::state::AppState;
use crate::prelude::Result;

pub async fn build_routes() -> Result<Router> {
    let state = AppState::new().await?;
    Ok(Router::new()
        .route("/healthz/", get(healthz))
        .layer(from_fn_with_state(
            state.clone(),
            middlewares::authn::authenticate,
        ))
        .route("/livez/", get(livez))
        .with_state(state))
}

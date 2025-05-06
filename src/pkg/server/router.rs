use axum::middleware::from_fn_with_state;
use axum::{
    routing::get,
    Router,
};

use crate::prelude::Result;
use super::handlers::probes::{healthz, livez};
use super::middlewares;
use super::state::AppState;


pub async fn build_routes() -> Result<Router> {
    let state = AppState::new().await?;
    Ok(Router::new()
        .route("/livez/", get(livez))
        .route("/healthz/", get(healthz))
        .layer(from_fn_with_state(state.clone(), middlewares::authn::authenticate))
        .with_state(state))
}

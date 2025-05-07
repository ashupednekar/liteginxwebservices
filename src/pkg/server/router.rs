use axum::middleware::from_fn_with_state;
use axum::{Router, routing::get};

use super::handlers::probes::{healthz, livez};
use super::middlewares;
use super::state::AppState;
use crate::prelude::Result;

pub async fn build_routes() -> Result<Router> {
    let state = AppState::new().await?;
    Ok(Router::new()
        //home
        //containers
        //functions
        //repositories
        //endpoints
        //buckets
        //metrics
        .layer(from_fn_with_state(
            state.clone(),
            middlewares::authn::authenticate,
        ))
        .route("/healthz/", get(healthz))
        .route("/livez/", get(livez))
        .with_state(state))
}

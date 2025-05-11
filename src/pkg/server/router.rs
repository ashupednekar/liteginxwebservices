use axum::middleware::from_fn_with_state;
use axum::{Router, routing::get};

use super::handlers::probes::{healthz, livez};
use super::handlers::ui::{buckets, containers, functions, home, verify};
use super::middlewares::authn;
use super::state::AppState;
use crate::prelude::Result;

pub async fn build_routes() -> Result<Router> {
    let state = AppState::new().await?;
    let app = Router::new()
        .route("/", get(home))
        .route("/buckets", get(buckets))
        .route("/containers", get(containers))
        .route("/functions", get(functions))
        .nest("/auth", Router::new().route("/verify", get(verify)))
        .layer(from_fn_with_state(state.clone(), authn::authenticate))
        .route("/healthz", get(healthz))
        .route("/livez", get(livez))
        .with_state(state);

    Ok(app)
}

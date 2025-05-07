use axum::middleware::from_fn_with_state;
use axum::{Router, routing::get};

use super::handlers::probes::{healthz, livez};
use super::handlers::ui::{buckets, containers, functions, home};
use super::state::AppState;
use crate::prelude::Result;

pub async fn build_routes() -> Result<Router> {
    let state = AppState::new().await?;
    let app = Router::new()
        .nest(
            "/lws",  
            Router::new()
                .route("/", get(home))
                .route("/buckets", get(buckets))
                .route("/containers", get(containers))
                .route("/functions", get(functions))
        )
        //.layer(from_fn_with_state(
        //    state.clone(),
        //    middlewares::authn::authenticate,
        //))
        .route("/healthz", get(healthz))
        .route("/livez", get(livez))
        .with_state(state);

    Ok(app)
}

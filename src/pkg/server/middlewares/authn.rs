use axum::{extract::{Request, State}, middleware::Next, response::Response};

use crate::pkg::server::state::AppState;



pub async fn authenticate(
    State(state): State<AppState>,
    request: Request,
    next: Next
) -> Response{
    next.run(request).await
}

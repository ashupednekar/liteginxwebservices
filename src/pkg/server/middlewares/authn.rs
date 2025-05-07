use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode, header::COOKIE},
    middleware::Next,
    response::Response,
};
use axum_extra::extract::CookieJar;
use sqlx::query;
use standard_error::{StandardError, Status};

use crate::{pkg::server::state::AppState, prelude::Result};

#[derive(Debug)]
pub struct UserDetails {
    pub username: String,
}

pub async fn authenticate(
    State(state): State<AppState>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response> {
    match CookieJar::from_headers(&headers)
        .get("_Host_lwsuser")
        .filter(|c| !c.value().is_empty())
    {
        Some(token) => {
            if let None = query("SELECT 1 FROM tokens WHERE token = $1 AND status = 'verified'")
                .bind(token.value().to_owned())
                .fetch_optional(&*state.db_pool)
                .await?
            {
                tracing::debug!("token not valid, autentication denied");
                return Err(StandardError::new("ERR-AUTH-001"));
            }
        }
        None => {
            tracing::debug!("token missing, autentication denied");
            return Err(StandardError::new("ERR-AUTH-001").code(StatusCode::UNAUTHORIZED));
        }
    };
    let details = UserDetails {
        username: "ashupednekar".into(),
    };
    request.extensions_mut().insert(Arc::new(details));
    Ok(next.run(request).await)
}

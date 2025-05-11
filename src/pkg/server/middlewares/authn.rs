use std::sync::Arc;

use askama::Template;
use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode, header::COOKIE},
    middleware::Next,
    response::Response,
};
use axum_extra::extract::CookieJar;
use sqlx::query;
use standard_error::{HtmlRes, StandardError, Status};

use crate::{
    pkg::server::{state::AppState, uispec::Verify},
    prelude::Result,
};

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
    let email = "ashupednekar49@gmail.com";
    match CookieJar::from_headers(&headers)
        .get("_Host_lwsuser")
        .filter(|c| !c.value().is_empty())
    {
        Some(token) => {
            
        }
        None => {
            tracing::debug!("token missing, autentication denied");
            return Err(StandardError::new("ERR-AUTH-001")
                .code(StatusCode::UNAUTHORIZED)
                .template(Verify { email }.render()?));
        }
    };
    let details = UserDetails {
        username: "ashupednekar".into(),
    };
    request.extensions_mut().insert(Arc::new(details));
    Ok(next.run(request).await)
}

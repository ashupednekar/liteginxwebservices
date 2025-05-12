use std::sync::Arc;

use askama::Template;
use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode, header::COOKIE},
    middleware::Next,
    response::Response,
};
use axum_extra::extract::CookieJar;
use standard_error::{HtmlRes, StandardError, Status};

use crate::{
    pkg::{
        internal::auth::tokens::AuthToken,
        server::{state::AppState, uispec::Verify},
    },
    prelude::Result,
};


pub async fn authenticate(
    State(state): State<AppState>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response> {
    let jar = CookieJar::from_headers(&headers);
    let maybe_cookie = jar.get("_Host_lws_token").filter(|c| !c.value().is_empty());
    if let Some(cookie) = maybe_cookie {
        match AuthToken::check_token_validity(state.clone(), cookie.value()).await {
            Ok(user) => {
                request.extensions_mut().insert(Arc::new(user));
                return Ok(next.run(request).await);
            }
            Err(_) => {
                tracing::warn!("token not valid, authentication denied");
                return Err(StandardError::new("ERR-AUTH-001")
                    .code(StatusCode::UNAUTHORIZED)
                    .template(Verify { }.render()?));
            }
        }
    }
    tracing::warn!("token missing, authentication denied");
    Err(StandardError::new("ERR-AUTH-001")
        .code(StatusCode::UNAUTHORIZED)
        .template(Verify { }.render()?))
}

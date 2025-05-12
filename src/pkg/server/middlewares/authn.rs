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

pub struct UserDetails {
    pub username: String,
}

pub async fn authenticate(
    State(state): State<AppState>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response> {
    let email = headers.get("_Host_lws_email").and_then(|v|v.to_str().ok());
    let username = headers.get("_Host_lws_username").and_then(|v|v.to_str().ok());
    let jar = CookieJar::from_headers(&headers);
    let maybe_cookie = jar.get("_Host_lws_token").filter(|c| !c.value().is_empty());

    if let Some(cookie) = maybe_cookie {
        match AuthToken::verify(state.clone(), cookie.value(), email, username).await {
            Ok(_) => {
                let details = UserDetails {
                    username: username.unwrap().into(),
                };
                request.extensions_mut().insert(Arc::new(details));
                return Ok(next.run(request).await);
            }
            Err(_) => {
                return Err(StandardError::new("ERR-AUTH-001")
                    .code(StatusCode::UNAUTHORIZED)
                    .template(Verify { email: email.unwrap() }.render()?));
            }
        }
    }

    tracing::debug!("token missing, authentication denied");
    AuthToken::issue(state, email, username).await?;
    Err(StandardError::new("ERR-AUTH-001")
        .code(StatusCode::UNAUTHORIZED)
        .template(Verify { email: email.unwrap() }.render()?))
}

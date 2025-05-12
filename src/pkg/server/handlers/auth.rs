use std::sync::Arc;

use axum::{
    extract::State, http::{header::SET_COOKIE, HeaderMap, HeaderValue}, response::{AppendHeaders, Html, IntoResponse}, Extension, Form
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use serde::Deserialize;
use validator::Validate;

use crate::{pkg::{internal::auth::tokens::{AuthToken, User}, server::state::AppState}, prelude::Result};


#[derive(Deserialize)]
pub struct SignupInput {
    pub email: String,
    pub name: String
}


#[derive(Deserialize, Validate)]
pub struct VerifyInput {
    #[validate(length(equal = 6))]
    pub code: String,
}


pub async fn signup(
    State(state): State<AppState>,
    Form(input): Form<SignupInput>,
) -> Result<impl IntoResponse> {
    let user = AuthToken::issue(state, &input.email, &input.name).await?;
    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        HeaderValue::from_str(&format!("_Host_lws_email={}", &user.email))?
    );
    Ok(headers)
}


pub async fn verify(
    Extension(user): Extension<Arc<User>>,
    Form(input): Form<VerifyInput>,
) -> Result<(HeaderMap, Html<String>)> {
    if input.code != "121212" {
        let headers = HeaderMap::new();
        return Ok((headers, Html(
            r#"<div id='code-error' class='text-red-500 text-center text-sm mt-2'>Invalid code. Please try again.</div>"#.to_string()
        )));
    }
    tracing::debug!("details: {:?}", &user);
    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        HeaderValue::from_static("_Host_lwsuser=snlkdjn"),
    );
    headers.insert(
        SET_COOKIE,
        HeaderValue::from_static("_Host_lwsuser=snlkdjn"),
    );
    headers.insert(
        SET_COOKIE,
        HeaderValue::from_static("_Host_lwsuser=snlkdjn"),
    );
    Ok((
        headers,
        Html(
            "<div class='text-green-600 text-center text-lg'>Verification successful!</div>"
                .to_string(),
        ),
    ))
}

use axum::{
    http::{HeaderMap, HeaderValue, header::SET_COOKIE},
    response::{AppendHeaders, Html},
};
use serde::Deserialize;
use validator::Validate;

use crate::prelude::Result;

#[derive(Deserialize, Validate)]
pub struct VerifyInput {
    #[validate(length(equal = 6))]
    pub code: String,
}

pub async fn verify(
    axum::Form(input): axum::Form<VerifyInput>,
) -> Result<(HeaderMap, Html<String>)> {
    if input.code != "121212" {
        // Return only the error message div
        let headers = HeaderMap::new();
        return Ok((headers, Html(
            r#"<div id='code-error' class='text-red-500 text-center text-sm mt-2'>Invalid code. Please try again.</div>"#.to_string()
        )));
    }
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

use std::sync::Arc;

use axum::{extract::State, http::HeaderMap, Extension, Form, Json};
use axum_extra::extract::CookieJar;
use serde::Deserialize;
use serde_json::{json, Value};
use standard_error::StandardError;
use validator::Validate;

use crate::{pkg::{internal::{auth::User, project::Project}, server::state::AppState}, prelude::Result};


#[derive(Deserialize, Validate)]
pub struct ProjectInput{
    #[validate(length(min = 1, message = "Field cannot be empty"))]
    pub name: String,
    #[validate(length(min = 1, message = "Field cannot be empty"))]
    pub description: String
}


pub async fn create(
    State(state): State<AppState>,
    Extension(user): Extension<Arc<User>>,
    Form(input): Form<ProjectInput>,
) ->Result<Json<Project>>{
    let project = Project::create(&state, &input.name, &input.description).await?;
    let code = project.invite(&state, &user.user_id).await?;
    Project::accept_invite(&state, &code).await?;
    Ok(Json(project))
}

#[derive(Deserialize, Validate)]
pub struct InviteInput{
    #[validate(length(min = 1, message = "Field cannot be empty"))]
    pub email: String
}


pub async fn invite(
    State(state): State<AppState>,
    headers: HeaderMap,
    Form(input): Form<InviteInput>
) -> Result<Json<Value>>{
    let jar = CookieJar::from_headers(&headers);
    let project_id = match jar.get("current_project").filter(|c| !c.value().is_empty()){
        Some(p) => p.value(),
        None => {return Err(StandardError::new("ERR-PROJ-001"));}
    };
    let project = Project::retrieve(&state, project_id).await?;
    let user = match User::retrieve(&state, &input.email).await?{
        Some(u) => u,
        None => {
            let (name, _) = input.email.split_once("@").unwrap_or(("unknown", ""));
            User::create(&state, &input.email, &name).await?
        }
    };
    let code = project.invite(&state, &user.user_id).await?;
    Ok(Json(json!({
        "code": code
    })))
}

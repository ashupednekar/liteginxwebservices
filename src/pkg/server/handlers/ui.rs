use askama::Template;
use axum::{extract::{Query, State}, response::Html};
use standard_error::{Interpolate, StandardError};

use crate::{
    pkg::{
        internal::project::Project,
        server::{state::AppState, uispec::{Buckets, Containers, Functions, Home, Metrics, ShowInvite, Verify}},
    },
    prelude::Result,
};

pub async fn buckets() -> Html<String> {
    let template = Buckets { username: "Ashu" };
    Html(template.render().unwrap())
}

pub async fn containers() -> Html<String> {
    let template = Containers { username: "Ashu" };
    Html(template.render().unwrap())
}

pub async fn functions() -> Html<String> {
    let template = Functions { username: "Ashu" };
    Html(template.render().unwrap())
}

pub async fn show_invite(
    Query(invite_code): Query<String>,
    State(state): State<AppState>
) -> Result<Html<String>>{
    let (project, inviter) = Project::invite_details(&state, &invite_code).await?;
    let template = ShowInvite {
        inviter: &inviter,
        project_name: &project.name,
        project_description: &project.description,
        invite_id: &invite_code
    };
    tracing::debug!("showing invite: {:?}", &template);
    Ok(Html(template.render()?))
}

pub async fn home(
    State(state): State<AppState>
) ->Result<Html<String>> {
    let projects = Project::list(&state).await?;
    tracing::debug!("projects: {:?}", &projects);
    let metrics = Metrics {
        containers: 2,
        functions: 5,
        buckets: 3,
        total_requests: 1200000,
    };

    let template = Home {
        username: "Ashu",
        projects,
        metrics,
    };

    Ok(Html(template.render()?))
}

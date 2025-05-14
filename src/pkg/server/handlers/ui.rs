use askama::Template;
use axum::response::Html;
use standard_error::{Interpolate, StandardError};

use crate::{
    pkg::{
        internal::project::Project,
        server::uispec::{Buckets, Containers, Functions, Home, Metrics, Verify},
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

pub async fn home() -> Result<Html<String>> {
    let projects = vec![];

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

    Ok(Html(template.render().map_err(|e| {
        StandardError::new("ERR-RENDER").interpolate_err(e.to_string())
    })?))
}

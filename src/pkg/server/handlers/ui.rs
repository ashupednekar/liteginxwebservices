use askama::Template;
use axum::response::Html;

use crate::pkg::server::uispec::{Buckets, Containers, Functions, Home};

pub async fn buckets() -> Html<String> {
    let template = Buckets { username: "ashu" };
    Html(template.render().unwrap())
}

pub async fn containers() -> Html<String> {
    let template = Containers { username: "ashu" };
    Html(template.render().unwrap())
}

pub async fn functions() -> Html<String> {
    let template = Functions { username: "ashu" };
    Html(template.render().unwrap())
}

pub async fn home() -> Html<String> {
    let template = Home { username: "ashu" };
    tracing::debug!("home route");
    Html(template.render().unwrap())
}

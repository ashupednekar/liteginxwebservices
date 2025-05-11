use askama::Template;
use axum::response::Html;
use standard_error::{Interpolate, StandardError};

use crate::{
    pkg::server::uispec::{Buckets, Containers, Functions, Home, Metrics, Project, Verify},
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
    let projects = vec![
        Project {
            id: "b7e8a1c2-1111-4a2b-8c3d-1234567890ab".to_string(),
            name: "DevProj1",
            description: Some("dummy proj 1"),
        },
        Project {
            id: "b7e8a1c2-2222-4a2b-8c3d-1234567890ab".to_string(),
            name: "E-commerce",
            description: Some("API services for e-commerce platform"),
        },
        Project {
            id: "b7e8a1c2-3333-4a2b-8c3d-1234567890ab".to_string(),
            name: "Retail",
            description: Some("retail services"),
        },
    ];

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

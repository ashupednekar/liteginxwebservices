use reqwest::StatusCode;
use serde::Serialize;
use standard_error::{Interpolate, StandardError};

use crate::{conf::settings, prelude::Result};


#[derive(Serialize)]
struct CreateRepoRequest<'a> {
    name: &'a str,
    description: &'a str,
    private: bool,
}

pub async fn create_new_repo(name: &str, description: &str) -> Result<()>{
    let client = reqwest::Client::new();
    let headers = |req: reqwest::RequestBuilder| {
        req.header("Authorization", format!("Bearer {}", &settings.git_token))
            .header("User-Agent", "reqwest")
            .header("Accept", "application/vnd.github+json")
    };
    let check_url = format!("https://api.github.com/repos/{}/{}", &settings.git_user, name);
    let res = headers(client.get(&check_url)).send().await?;
    if res.status() == StatusCode::OK {
        tracing::info!("📦 Repo exists. Deleting...");
        let delete_url = check_url.clone();
        let delete_res = headers(client.delete(&delete_url)).send().await?;
        if delete_res.status().is_success() {
            tracing::info!("🗑️ Repo deleted successfully.");
        } else {
            tracing::error!("❌ Failed to delete repo: {}", delete_res.text().await?);
            return Ok(());
        }
    }
    tracing::info!("🚀 Creating new repo...");
    let create_url = "https://api.github.com/user/repos";
    let body = CreateRepoRequest {
        name,
        description,
        private: false,
    };
    let create_res = headers(client.post(create_url))
        .json(&body)
        .send()
        .await
        ?;

    if create_res.status().is_success() {
        tracing::info!("✅ Repo created successfully.");
    } else {
        tracing::error!("❌ Failed to create repo: {}", create_res.text().await?);
    }
    Ok(())
}

pub async fn delete_repo(name: &str) -> Result<()> {
    let client = reqwest::Client::new();
    let url = format!("https://api.github.com/repos/{}/{}", &settings.git_user, &name);
    let res = client
        .delete(&url)
        .header("Authorization", format!("Bearer {}", &settings.git_token))
        .header("User-Agent", "reqwest") 
        .header("Accept", "application/vnd.github+json")
        .send()
        .await?;
    match res.status() {
        StatusCode::NO_CONTENT => {
            println!("✅ Repository deleted successfully.");
        }
        StatusCode::NOT_FOUND => {
            println!("⚠️ Repository not found.");
        }
        status => {
            eprintln!("❌ Failed to delete repo ({}): {}", status, res.text().await?);
        }
    }
    Ok(())
}


#[cfg(test)]
mod tests{
    use tracing_test::traced_test;
    use crate::{prelude::Result, pkg::{internal::{auth::User, project::Project}, server::state::AppState}};

    #[tokio::test]
    #[traced_test]
    async fn test_create_repo() -> Result<()>{
        let state = AppState::new().await?;
        let user = User::create(&state, "jane@a.com", "jane").await?;
        Project::create(&state, "janetestone", "jane's first project", &user.user_id).await?;
        Ok(())
    }
}

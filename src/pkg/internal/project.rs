use std::sync::Arc;

use crate::{pkg::server::state::AppState, prelude::Result};
use futures::future::try_join_all;
use serde::Serialize;
use sqlx::prelude::{FromRow, Type};
use standard_error::StandardError;
use tokio::try_join;

#[derive(Debug, Type)]
#[sqlx(type_name = "invite_status", rename_all = "lowercase")]
pub enum InviteStatus {
    Pending,
    Accepted,
    Expired,
}

#[derive(FromRow, Serialize, Debug)]
pub struct Project {
    pub project_id: String,
    pub name: String,
    pub description: String,
}

#[derive(FromRow, Debug)]
pub struct Access {
    pub user_id: String,
    pub project_id: String,
}

impl Project {
    pub async fn create(state: Arc<AppState>, name: &str, description: &str) -> Result<Self> {
        let project = sqlx::query_as!(
            Project,
            r#"
            INSERT INTO projects (name, description)
            VALUES ($1, $2)
            ON CONFLICT (name) DO NOTHING
            RETURNING project_id, name, description
            "#,
            name,
            description
        )
        .fetch_one(&*state.db_pool)
        .await?;
        Ok(project)
    }

    pub async fn list(state: Arc<AppState>) -> Result<Vec<Self>> {
        let projects = sqlx::query_as!(
            Project,
            "select project_id, name, description from projects"
        )
        .fetch_all(&*state.db_pool)
        .await?;
        Ok(projects)
    }

    pub async fn retrieve(state: Arc<AppState>, project_id: &str) -> Result<Self> {
        let project = sqlx::query_as!(
            Project,
            "select project_id, name, description from projects where project_id = $1",
            &project_id
        )
        .fetch_one(&*state.db_pool)
        .await?;
        Ok(project)
    }

    pub async fn assign(&self, state: Arc<AppState>, users: Vec<&str>) -> Result<()> {
        let futures = users.iter().map(|user_id| {
            sqlx::query!(
                r#"
                INSERT INTO project_access (project_id, user_id)
                VALUES ($1, $2)
                on conflict (project_id, user_id) do nothing
                "#,
                self.project_id,
                *user_id
            )
            .execute(&*state.db_pool)
        });
        try_join_all(futures).await?;
        Ok(())
    }


    pub async fn unassign(&self, state: Arc<AppState>, users: Vec<&str>) -> Result<()> {
        let futures = users.iter().map(|user_id| {
            sqlx::query!(
                r#"
                delete from project_access
                where project_id = $1 and user_id = $2
                "#,
                self.project_id,
                *user_id
            )
            .execute(&*state.db_pool)
        });
        try_join_all(futures).await?;
        Ok(())
    }


    pub async fn delete(&self, state: Arc<AppState>) -> Result<()> {
        sqlx::query!(
            "delete from project_access where project_id = $1",
            &self.project_id
        )
        .execute(&*state.db_pool)
        .await?;
        sqlx::query!(
            "delete from projects where project_id = $1",
            &self.project_id
        )
        .execute(&*state.db_pool)
        .await?;
        Ok(())
    }
}




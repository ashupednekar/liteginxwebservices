use crate::{pkg::server::state::AppState, prelude::Result};
use serde::Serialize;
use sqlx::prelude::{FromRow, Type};
use standard_error::StandardError;
use uuid::Uuid;

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
    pub async fn create(state: &AppState, name: &str, description: &str) -> Result<Self> {
        let project = sqlx::query_as!(
            Project,
            r#"
            INSERT INTO projects (name, description, project_id)
            VALUES ($1, $2, $3)
            ON CONFLICT (name) DO NOTHING
            RETURNING project_id, name, description
            "#,
            name,
            description,
            Uuid::new_v4().to_string()
        )
        .fetch_one(&*state.db_pool)
        .await?;
        Ok(project)
    }

    pub async fn list(state: &AppState) -> Result<Vec<Self>> {
        let projects = sqlx::query_as!(
            Project,
            "select project_id, name, description from projects"
        )
        .fetch_all(&*state.db_pool)
        .await?;
        Ok(projects)
    }

    pub async fn retrieve(state: &AppState, project_id: &str) -> Result<Self> {
        let project = sqlx::query_as!(
            Project,
            "select project_id, name, description from projects where project_id = $1",
            &project_id
        )
        .fetch_one(&*state.db_pool)
        .await?;
        Ok(project)
    }

    pub async fn delete(&self, state: &AppState) -> Result<()> {
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


    pub async fn invite(&self, state: &AppState, user_id: &str) -> Result<String> {
        let invite_code = sqlx::query_scalar!(
            r#"
            insert into project_access (invite_id, project_id, user_id, expiry)
            values ($1, $2, $3, NOW() + interval '1 hour')
            on conflict (project_id, user_id) do update 
            set expiry = NOW() + INTERVAL '1 hour'
            returning invite_id
            "#,
            Uuid::new_v4().to_string(),
            &self.project_id,
            user_id
        )
        .fetch_one(&*state.db_pool)
        .await?;
        Ok(invite_code)
    }

    pub async fn accept_invite(state: &AppState, invite_code: &str) -> Result<()>{
        match sqlx::query!(
            "update project_access set status = $2 where invite_id = $1 and expiry > NOW()",
            &invite_code,
            InviteStatus::Accepted as _
        ).fetch_optional(&*state.db_pool).await?{
            Some(_) => {},
            None => {return Err(StandardError::new("ERR-INVITE-EXPIRED"));}
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests{
    use tracing_test::traced_test;
    use crate::pkg::internal::auth::User;

    use super::*;

    #[tokio::test]
    #[traced_test]
    async fn test_project_crud() -> Result<()>{
        let state = AppState::new().await?;
        let before = Project::list(&state).await?.len();
        Project::create(&state, "proj1", "first project").await?;
        Project::create(&state, "proj2", "second project").await?;
        Project::create(&state, "proj3", "third project").await?;
        let projects = Project::list(&state).await?;
        assert_eq!(projects.len() - before, 3);
        let project_id = projects[0].project_id.clone();
        let project = Project::retrieve(&state, &project_id).await?;
        project.delete(&state).await?;
        Ok(())
    }

    #[tokio::test]
    #[traced_test]
    async fn test_project_invite_accept() -> Result<()>{
        let state = AppState::new().await?;
        let user = User::create(&state, "ashupednekar49@gmail.com", "Ashu Pednekar").await?;
        let project = Project::create(&state, "proj", "project description").await?;
        let invite_code = project.invite(&state, &user.user_id).await?;
        Project::accept_invite(&state, &invite_code).await?;
        Ok(())
    }


    #[tokio::test]
    #[traced_test]
    async fn test_project_invite_expiry() -> Result<()>{
        Ok(())
    }
}




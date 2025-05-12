use crate::{
    pkg::server::{state::AppState, uispec::Verify},
    prelude::Result,
};

use askama::Template;
use axum::http::StatusCode;
use rand::{Rng, distr::Alphanumeric};
use sqlx::{
    prelude::{FromRow, Type},
    types::time::OffsetDateTime,
};
use standard_error::{HtmlRes, StandardError, Status};
use uuid::Uuid;

#[derive(Debug, Type)]
#[sqlx(type_name = "token_status", rename_all = "lowercase")]
pub enum TokenStatus {
    Pending,
    Verified,
    Expired,
}

#[derive(FromRow, Debug)]
pub struct AuthToken {
    pub token: Uuid,
    pub user_id: String,
    pub code: String,
    pub expiry: OffsetDateTime,
    pub status: TokenStatus,
}

#[derive(FromRow, Debug)]
pub struct User {
    pub user_id: String,
    pub email: String,
    pub name: Option<String>,
}

impl AuthToken {
    fn generate_code() -> String {
        rand::rng()
            .sample_iter(&Alphanumeric)
            .take(6)
            .map(char::from)
            .collect()
    }

    pub async fn issue(state: AppState, email: &str, name: &str) -> Result<User> {
        let pool = &*state.db_pool;
        let maybe_user = sqlx::query_as!(
                User,
                r#"SELECT user_id, email, name FROM users WHERE email = $1"#,
                email
            )
            .fetch_optional(pool)
            .await?;
        let user = match maybe_user {
            Some(user) => user,
            None => {
                let user_id = Uuid::new_v4().to_string();
                match sqlx::query_as!(
                    User,
                    r#"
                    INSERT INTO users (user_id, email, name)
                    VALUES ($1, $2, $3)
                    ON CONFLICT (email) DO NOTHING
                    RETURNING user_id, email, name 
                    "#,
                    user_id,
                    email,
                    name
                )
                .fetch_optional(pool)
                .await?
                {
                    Some(user) => user,
                    None => {
                        return Err(StandardError::new("ERR-AUTH-004")
                            .code(StatusCode::INTERNAL_SERVER_ERROR));
                    }
                }
            }
        };

        let code = Self::generate_code();

        sqlx::query!(
            r#"
            INSERT INTO tokens (user_id, code, expiry, status)
            VALUES ($1, $2, NOW() + interval '1 hour', $3)
            "#,
            user.user_id,
            code,
            TokenStatus::Pending as _
        )
        .execute(pool)
        .await?;

        tracing::info!("Issued token for user {}", user.user_id);
        Ok(user)
    }

    pub async fn check_code(token: AuthToken, code: &str) -> Result<()>{
        if token.code == code {Ok(())} else {Err(StandardError::new("ERR-AUTH-003"))}
    }

    pub async fn check_token_validity(
        state: AppState,
        token_str: &str,
    ) -> Result<User> {
        let pool = &*state.db_pool;
        let token_str = token_str
            .parse::<Uuid>()
            .map_err(|_| StandardError::new("ERR-AUTH-002"))?;

       tracing::info!("Verifying token: {}", token_str);
        let result = sqlx::query_as!(
            AuthToken,
            r#"
            SELECT token, user_id, code, expiry, status as "status: _"
            FROM tokens
            WHERE token = $1
            "#,
            &token_str
        )
        .fetch_optional(pool)
        .await;
        if let Ok(Some(token)) = result{
             let user = sqlx::query_as!(
                    User,
                    r#"SELECT user_id, email, name FROM users WHERE user_id = $1"#,
                    token.user_id 
                )
                .fetch_one(pool)
                .await?;
            Ok(user)
        }else{
            Err(StandardError::new("ERR-AUTH-001"))
        }
    }
}

#[cfg(test)]
mod tests {
    use tracing_test::traced_test;
    use uuid::Uuid;

    use crate::{
        pkg::{internal::auth::tokens::AuthToken, server::state::AppState},
        prelude::Result,
    };

    #[traced_test]
    #[tokio::test]
    async fn test_verify() -> Result<()> {
        let state = AppState::new().await?;
        let token = Uuid::new_v4();
        let _ = AuthToken::check_token_validity(
            state,
            &token.to_string(),
        )
        .await;
        Ok(())
    }
}

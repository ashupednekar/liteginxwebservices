use crate::{
    pkg::server::{state::AppState, uispec::Verify},
    prelude::Result,
};

use askama::Template;
use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use rand::{Rng, distr::Alphanumeric};
use serde::Deserialize;
use sqlx::{
    PgPool,
    prelude::{FromRow, Type},
    query,
    types::time::OffsetDateTime,
};
use standard_error::{HtmlRes, StandardError, Status};
use uuid::Uuid;
use validator::Validate;

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
    pub user_id: Uuid,
    pub code: String,
    pub expiry: OffsetDateTime,
    pub status: TokenStatus,
}

#[derive(FromRow, Debug)]
pub struct User {
    pub email: String,
    pub user_id: String,
}

impl AuthToken {
    fn generate_code() -> String {
        rand::rng()
            .sample_iter(&Alphanumeric)
            .take(6)
            .map(char::from)
            .collect()
    }

    pub async fn issue(state: AppState, email: Option<&str>, username: Option<&str>) -> Result<()> {
        let pool = &*state.db_pool;
        let email = email.ok_or_else(|| StandardError::new("ERR-AUTH-003"))?;
        let maybe_user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (email, username)
            VALUES ($1, $2)
            ON CONFLICT (email) DO NOTHING
            RETURNING email, user_id
            "#,
            email,
            username
        )
        .fetch_optional(pool)
        .await?;

        let user = match maybe_user {
            Some(user) => user,
            None => {
                match sqlx::query_as!(
                    User,
                    r#"SELECT user_id, email FROM users WHERE email = $1"#,
                    email
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
            user.user_id
                .parse::<Uuid>()
                .map_err(|_| StandardError::new("ERR-AUTH-002"))?,
            code,
            TokenStatus::Pending as _
        )
        .execute(pool)
        .await?;

        tracing::info!("Issued token for user {}", user.user_id);
        Err(StandardError::new("ERR-AUTH-001")
            .code(StatusCode::UNAUTHORIZED)
            .template(Verify { email }.render()?))
    }

    pub async fn verify(
        state: AppState,
        token_str: &str,
        email: Option<&str>,
        username: Option<&str>,
    ) -> Result<()> {
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
        .await?;

        match result {
            Some(token) => {
                if let TokenStatus::Verified = token.status {
                    Ok(())
                } else {
                    AuthToken::issue(state, email, username).await
                }
            }
            None => AuthToken::issue(state, email, username).await,
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
        let _ = AuthToken::verify(
            state,
            &token.to_string(),
            Some("ashupednekar49@gmail.com"),
            Some("username"),
        )
        .await;
        Ok(())
    }
}

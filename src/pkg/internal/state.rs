use crate::{pkg::utils::db::get_pool as db_pool, prelude::Result};
use std::sync::Arc;
use sqlx::PgPool;


#[derive(Debug)]
pub struct AppState{
    pub db_pool: Arc<PgPool>
}

impl AppState{
    pub async fn new() -> Result<AppState>{
        Ok(AppState { db_pool: Arc::new(db_pool()?) })
    }
}

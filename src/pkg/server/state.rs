use std::sync::Arc;
use sqlx::PgPool;



use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::{prelude::Result, conf::settings};



pub fn db_pool() -> Result<Pool<Postgres>> {
    let pool = PgPoolOptions::new()
        .max_connections(settings.database_pool_max_connections)
        .connect_lazy(&settings.database_url)?;
    Ok(pool)
}

#[derive(Debug, Clone)]
pub struct AppState{
    pub db_pool: Arc<PgPool>
}

impl AppState{
    pub async fn new() -> Result<AppState>{
        Ok(AppState { db_pool: Arc::new(db_pool()?) })
    }
}

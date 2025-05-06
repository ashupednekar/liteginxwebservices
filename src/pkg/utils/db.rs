use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::{prelude::Result, conf::settings};



pub fn get_pool() -> Result<Pool<Postgres>> {
    let pool = PgPoolOptions::new()
        .max_connections(settings.database_pool_max_connections)
        .connect_lazy(&settings.database_url)?;
    Ok(pool)
}

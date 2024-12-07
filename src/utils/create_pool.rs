use dotenvy;
use lazy_static::lazy_static;
use sqlx::{postgres::PgPool, Error};
use std::{env, sync::Arc};
use tokio::sync::{Mutex, OnceCell};
lazy_static! {
    static ref DB_POOL: OnceCell<Arc<PgPool>> = OnceCell::new();
}

pub async fn create_db_pool() -> Result<Arc<PgPool>, Error> {
    if let Some(pool) = DB_POOL.get() {
        return Ok(pool.clone());
    }
    let _ = dotenvy::from_filename(".env").ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL should be specified in the env");
    let pool = PgPool::connect(&db_url).await?;
    let pool = Arc::new(pool);
    DB_POOL.set(pool.clone()).map_err(|_| Error::PoolTimedOut)?;

    Ok(pool)
}

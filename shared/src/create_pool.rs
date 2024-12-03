use dotenvy;
use lazy_static::lazy_static;
use sqlx::{postgres::PgPool, Error};
use std::{env, sync::Arc};
use tokio::sync::Mutex;
lazy_static! {
    static ref DB_POOL: tokio::sync::OnceCell<Arc<Mutex<PgPool>>> = tokio::sync::OnceCell::new();
}

pub async fn create_db_pool() -> Result<Arc<Mutex<PgPool>>, Error> {
    if let Some(pool) = DB_POOL.get() {
        return Ok(pool.clone());
    }
    match dotenvy::from_filename("../.env") {
        Ok(_) => {
            println!("Loaded .env file");
            ()
        }
        Err(err) => return Err(Error::Configuration(err.to_string().into())),
    }
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL should be specified in the env");
    let pool = PgPool::connect(&db_url).await?;

    // Wrap the pool in Arc and Mutex, and store it in the static variable and Set the static variable with the initialized pool
    let pool = Arc::new(Mutex::new(pool));
    DB_POOL.set(pool.clone()).map_err(|_| Error::PoolTimedOut)?;

    Ok(pool)
}

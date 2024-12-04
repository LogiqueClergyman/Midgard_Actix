
use shared::{create_db_pool, run_migrations};
use std::sync::Arc;
use populate::scripts::cron_job::start_cron_job;
#[tokio::main]
async fn main() {
    let pool = create_db_pool().await.expect(
        "Failed to create database pool. Ensure the database server is running and accessible.",
    );
    let pool = Arc::new(pool);

    run_migrations(&*pool.lock().await).await.unwrap();

    start_cron_job(&pool).await;
}

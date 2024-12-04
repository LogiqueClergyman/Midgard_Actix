use crate::scripts::{depth_price_history, earnings_history, runepool_history, swaps_history};
use std::sync::Arc;
use tokio::time::{sleep, Duration};
pub async fn start_cron_job(pool: Arc<sqlx::PgPool>) {
    println!("Starting cron job");
    loop {
        if let Err(err) = depth_price_history::fetch_and_insert_data(&*pool).await {
            eprintln!("Error running depth_price_history: {}", err);
        }
        if let Err(err) = earnings_history::fetch_and_insert_data(&*pool).await {
            eprintln!("Error running earnings_history: {}", err);
        }
        if let Err(err) = runepool_history::fetch_and_insert_data(&*pool).await {
            eprintln!("Error running runepool_history: {}", err);
        }
        if let Err(err) = swaps_history::fetch_and_insert_data(&*pool).await {
            eprintln!("Error running swaps_history: {}", err);
        }
        sleep(Duration::from_secs(3600)).await;
    }
}

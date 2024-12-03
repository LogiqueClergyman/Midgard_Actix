use shared::{create_db_pool, run_migrations};
#[path = "./models/mod.rs"]
mod models;
#[path = "./scripts/mod.rs"]
mod scripts;
use scripts::{depth_price_history, earnings_history, runepool_history, swaps_history};

#[tokio::main]
async fn main() {
    let pool = create_db_pool().await.expect(
        "Failed to create database pool. Ensure the database server is running and accessible.",
    );
    let pool = pool.lock().await;
    let expression = "0 0 * * * *"; // Cron expression to run every hour on the hour
    let schedule = Schedule::from_str(expression).unwrap();
    // run_migrations(&pool).await.unwrap();
    // let script1 = depth_price_history::fetch_and_insert_data(&pool).await;
    // let script2 = earnings_history::fetch_and_insert_data(&pool).await;
    // let script3 = runepool_history::fetch_and_insert_data(&pool).await;
    // let script4 = swaps_history::fetch_and_insert_data(&pool).await;
    // let res = tokio::try_join!(script1, script2, script3, script4);
    // if let Err(e) = res {
    //     eprintln!("Error: {:?}", e);
    // }
    loop {
        let next_run = schedule.upcoming(Utc).next().unwrap();
        let duration_until_next_run = next_run.signed_duration_since(Utc::now()).to_std().unwrap();

        sleep(duration_until_next_run).await;

        let pool = pool.clone();
        let script1 = depth_price_history::fetch_and_insert_data(&pool).await;
        let script2 = earnings_history::fetch_and_insert_data(&pool).await;
        let script3 = runepool_history::fetch_and_insert_data(&pool).await;
        let script4 = swaps_history::fetch_and_insert_data(&pool).await;

        let res = tokio::try_join!(script1, script2, script3, script4);
        
        if let Err(e) = res {
            eprintln!("Error: {:?}", e);
        }
    }
}

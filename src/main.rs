// src/main.rs
#[path = "./fetch/mod.rs"]
pub mod fetch;
#[path = "./models/mod.rs"]
pub mod models;
#[path = "./routes/mod.rs"]
pub mod routes;
#[path = "./utils/mod.rs"]
pub mod utils;
use std::sync::Arc;

use actix_web::{web, App, HttpServer};
use utils::{create_pool::create_db_pool, migrations};
#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = match create_db_pool().await {
        Ok(pool) => pool,
        Err(err) => {
            eprintln!("Error creating pool: {}", err);
            return Err("Error creating pool".into());
        }
    };
    migrations::run_migrations(&pool).await?;
    // tokio::spawn({
    //     let pool = Arc::clone(&pool);
    //     async move {
    //         fetch::cron_job::start_cron_job(pool).await;
    //     }
    // });
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let _ = HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .service(web::resource("/").to(|| async { "Hello, world!" }))
        // .route(
        //     "/depth_history",
        //     web::get().to({
        //         let pool = pool.clone();
        //         move |query: web::Query<models::depth_price_history::QueryParams>| {
        //             routes::depth_price_history::get_depth_price_history(
        //                 web::Data::new(pool.clone()),
        //                 query,
        //             )
        //         }
        //     }),
        // )
        // .route(
        //     "/runepool_history",
        //     web::get().to({
        //         let pool = pool.clone();
        //         move |query: web::Query<models::runepool_history::QueryParams>| {
        //             routes::runepool_history::get_runepool_history(
        //                 web::Data::new(pool.clone()),
        //                 query,
        //             )
        //         }
        //     }),
        // )
        // .route(
        //     "/swaps_history",
        //     web::get().to({
        //         let pool = pool.clone();
        //         move |query: web::Query<models::swaps_history::SwapQueryParams>| {
        //             routes::swaps_history::get_swap_history(web::Data::new(pool.clone()), query)
        //         }
        //     }),
        // )
        // .route(
        //     "/earnings_history",
        //     web::get().to({
        //         let pool = pool.clone();
        //         move |query: web::Query<models::earnings_history::EarningHistoryQueryParams>| {
        //             routes::earnings_history::get_earning_history(
        //                 web::Data::new(pool.clone()),
        //                 query,
        //             )
        //         }
        //     }),
        // )
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await;
    Ok(())
}

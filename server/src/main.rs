// src/main.rs
pub mod models;
pub mod routes;
use actix_web::{web, App, HttpServer};
use populate::scripts::cron_job::start_cron_job;
use routes::{
    depth_price_history::get_depth_price_history, earnings_history::get_earning_history,
    runepool_history::get_runepool_history, swaps_history::get_swap_history,
};
use shared::create_db_pool;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = match create_db_pool().await {
        Ok(pool) => pool,
        Err(err) => {
            eprintln!("Error creating pool: {}", err);
            return Err("Error creating pool".into());
        }
    };
    let pool_clone = pool.clone();

    let pool = web::Data::new(pool);

    // Spawn the cron job (no need for another runtime, Actix uses tokio)
    tokio::spawn(async move {
        println!("Starting cron job");
        start_cron_job(pool_clone).await;
    });
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let _ = HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .service(web::resource("/").to(|| async { "Hello, world!" }))
            .route(
                "/runepool_history",
                web::get().to({
                    let value = pool.clone();
                    move |query: web::Query<models::runepool_history::QueryParams>| {
                        get_runepool_history(value.clone(), query)
                    }
                }),
            )
            .route(
                "/depth_history",
                web::get().to({
                    let value = pool.clone();
                    move |query: web::Query<models::depth_price_history::QueryParams>| {
                        get_depth_price_history(value.clone(), query)
                    }
                }),
            )
            .route(
                "/swaps_history",
                web::get().to({
                    let value = pool.clone();
                    move |query: web::Query<models::swap_history::SwapQueryParams>| {
                        get_swap_history(value.clone(), query)
                    }
                }),
            )
            .route(
                "/earnings_history",
                web::get().to({
                    let value = pool.clone();
                    move |query: web::Query<models::earnings_history::EarningHistoryQueryParams>| {
                        get_earning_history(value.clone(), query)
                    }
                }),
            )
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await;
    Ok(())
}

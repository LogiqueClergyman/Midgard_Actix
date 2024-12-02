// src/main.rs
pub mod models;
pub mod routes;
use actix_web::{web, App, HttpServer};
use routes::runepool_history::{get_runepool_history, QueryParams};
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
    create_db_pool().await.unwrap();
    let pool = web::Data::new(pool.clone());

    let _ = HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .service(web::resource("/").to(|| async { "Hello, world!" }))
            .route(
                "/runepool_history",
                web::get().to({
                    let value = pool.clone();
                    move |query: web::Query<QueryParams>| get_runepool_history(value.clone(), query)
                }),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await;
    Ok(())
}

use chrono::NaiveDate;
use sqlx::Row;
use reqwest::Error;
use serde_json::Value;
pub async fn get_last_successful_entry_for_table(pool: &sqlx::PgPool, table_name: &str) -> i64 {
    let default_timestamp = NaiveDate::from_ymd_opt(2024, 10, 1)
        .unwrap_or_else(|| panic!("Invalid date"))
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .timestamp();

    let query = format!(
        "SELECT COALESCE(MAX(end_time), $1) as last_successful_entry FROM {}",
        table_name
    );
    println!("{:?}", query);
    match sqlx::query(&query)
        .bind(default_timestamp)
        .fetch_one(pool)
        .await
    {
        Ok(row) => row
            .get::<Option<i64>, _>("last_successful_entry")
            .unwrap_or(default_timestamp),
        Err(_) => default_timestamp,
    }
}

async fn fetch_and_parse_json(url: &str) -> Result<Value, String> {
    // Attempt to fetch the URL
    match reqwest::get(url).await {
        Ok(resp) => match resp.json().await {
            Ok(json) => Ok(json),
            Err(err) => {
                eprintln!("JSON parse error: {}", err);
                Err("JSON parse error".into())
            }
        },
        Err(err) => {
            eprintln!("Fetch error: {}", err);
            Err("Fetch error".into())
        }
    }
}

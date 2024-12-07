use crate::fetch::utils::get_last_successful_entry_for_table;
use crate::models::earnings_history::{EarningHistory, EarningHistoryNested};
use chrono::Utc;
use reqwest::get;
use serde_json::Value;
use sqlx::{Error, PgPool};
pub async fn fetch_and_insert_data(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let mut from_time = get_last_successful_entry_for_table(&pool, "earning_history").await;
    let end_time = Utc::now().timestamp() / 3600 * 3600;
    loop {
        let url = format!(
            "https://midgard.ninerealms.com/v2/history/earnings?interval=hour&from={}&count=100",
            from_time
        );
        println!("{:?}", url);

        let response: Value = get(&url).await.unwrap().json().await?;

        if let Some(intervals) = response["intervals"].as_array() {
            for entry in intervals {
                // Step 1: Create the EarningHistory record
                let interval_data: EarningHistory = serde_json::from_value(entry.clone())?;

                // Step 2: Insert the EarningHistory record into the database
                let earning_history_id = insert_earning_history(pool, interval_data).await?;

                // Step 3: Insert the EarningHistoryNested records for each pool
                for nested in entry["pools"].as_array().unwrap() {
                    let nested_data: EarningHistoryNested =
                        serde_json::from_value(nested.clone()).unwrap();
                    insert_earning_history_nested(pool, earning_history_id, nested_data).await?;
                }
            }
            let last_end_time = intervals
                .last()
                .and_then(|entry| entry["endTime"].as_str())
                .and_then(|end_time_str| end_time_str.parse::<i64>().ok())
                .unwrap_or(end_time);

            if last_end_time >= end_time {
                break;
            }
            from_time = last_end_time;
        } else {
            break;
        }
    }

    Ok(())
}
async fn insert_earning_history(pool: &PgPool, interval: EarningHistory) -> Result<i32, Error> {
    let query = "
    INSERT INTO earning_history (avg_node_count, block_rewards, bonding_earnings, earnings, end_time, liquidity_earnings, liquidity_fees, rune_price_usd, start_time)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) ON CONFLICT DO NOTHING RETURNING id";
    let earning_history_id: (i32,) = sqlx::query_as(query)
        .bind(interval.avg_node_count)
        .bind(interval.block_rewards)
        .bind(interval.bonding_earnings)
        .bind(interval.earnings)
        .bind(interval.end_time)
        .bind(interval.liquidity_earnings)
        .bind(interval.liquidity_fees)
        .bind(&interval.rune_price_usd)
        .bind(interval.start_time)
        .fetch_one(pool)
        .await?;
    Ok(earning_history_id.0)
}

async fn insert_earning_history_nested(
    pool: &PgPool,
    earning_history_id: i32,
    nested: EarningHistoryNested,
) -> Result<(), Error> {
    let query = "
    INSERT INTO earning_history_nested (earning_history_id, pool, asset_liquidity_fees, earnings, rewards, rune_liquidity_fees, saver_earning, total_liquidity_fees_rune)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8)";
    sqlx::query(query)
        .bind(earning_history_id)
        .bind(nested.pool)
        .bind(nested.asset_liquidity_fees)
        .bind(nested.earnings)
        .bind(nested.rewards)
        .bind(nested.rune_liquidity_fees)
        .bind(nested.saver_earning)
        .bind(nested.total_liquidity_fees_rune)
        .execute(pool)
        .await?;
    Ok(())
}

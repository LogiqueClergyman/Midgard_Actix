use sqlx::{PgPool, Error};
use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, Utc};
use reqwest::get;
use serde_json::Value;

#[derive(Deserialize, Serialize)]
struct EarningHistoryNestedData {
    pool: String,
    assetLiquidityFees: i64,
    earnings: i64,
    rewards: i64,
    runeLiquidityFees: i64,
    saverEarning: i64,
    totalLiquidityFeesRune: i64,
}

#[derive(Deserialize, Serialize)]
struct EarningHistoryData {
    avgNodeCount: f64,
    blockRewards: i64,
    bondingEarnings: i64,
    earnings: i64,
    endTime: i64,
    liquidityEarnings: i64,
    liquidityFees: i64,
    runePriceUsd: f64,
    startTime: i64,
    pool: Vec<i32>,
}

#[tokio::main]
async fn main() {
    let database_url = "postgres://user:password@localhost/database_name";
    let pool = PgPool::connect(database_url)
        .await
        .expect("Error creating pool");

    if let Err(e) = fetch_and_insert_data(&pool).await {
        eprintln!("Error occurred: {:?}", e);
    }
}

async fn fetch_and_insert_data(pool: &PgPool) -> Result<(), Error> {
    let start_date = NaiveDate::from_ymd_opt(2024, 10, 1)
        .unwrap_or_else(|| panic!("Invalid date"))
        .and_hms_opt(0, 0, 0)
        .unwrap();
    
    let mut from_time = start_date.timestamp();

    let last_successful_entry = get_last_successful_entry(pool).await.unwrap_or_else(|_| {
        println!("{:?}", start_date.timestamp());
        start_date.timestamp()
    });
    from_time = last_successful_entry;

    let end_time = Utc::now().timestamp();

    let mut count = 0;

    loop {
        let url = format!(
            "https://midgard.ninerealms.com/v2/history/earnings?interval=hour&from={}&count=100",
            from_time
        );
        println!("{:?}", url);

        let response: Value = get(&url).await.unwrap().json().await.unwrap();

        let mut nested_ids = Vec::new();
        for nested in response["intervals"].as_array().unwrap().last().unwrap()["pools"].as_array().unwrap() {
            let nested_data: EarningHistoryNestedData = serde_json::from_value(nested.clone()).unwrap();
            let nested_id = insert_pool(pool, &nested_data).await?;
            nested_ids.push(nested_id);
        }

        let interval_data = &response["intervals"].as_array().unwrap().last().unwrap();
        let interval = EarningHistoryData {
            avgNodeCount: interval_data["avgNodeCount"].as_str().unwrap().parse().unwrap(),
            blockRewards: interval_data["blockRewards"].as_str().unwrap().parse().unwrap(),
            bondingEarnings: interval_data["bondingEarnings"].as_str().unwrap().parse().unwrap(),
            earnings: interval_data["earnings"].as_str().unwrap().parse().unwrap(),
            endTime: interval_data["endTime"].as_str().unwrap().parse().unwrap(),
            liquidityEarnings: interval_data["liquidityEarnings"].as_str().unwrap().parse().unwrap(),
            liquidityFees: interval_data["liquidityFees"].as_str().unwrap().parse().unwrap(),
            runePriceUsd: interval_data["runePriceUSD"].as_str().unwrap().parse().unwrap(),
            startTime: interval_data["startTime"].as_str().unwrap().parse().unwrap(),
            pool: nested_ids
        };

        // Insert into earning_intervals table and store pool ids
        let interval_id = insert_earning_interval(pool, &interval, &nested_ids).await?;

        let last_end_time = interval.endTime;

        if last_end_time >= end_time {
            break;
        }

        from_time = last_end_time;
    }

    Ok(())
}

async fn get_last_successful_entry(pool: &PgPool) -> Result<i64, Error> {
    let query = "SELECT MAX(end_time) FROM earning_intervals";
    let row: Option<(i64,)> = sqlx::query_as(query).fetch_optional(pool).await?;
    Ok(row.map_or(0, |r| r.0))
}

async fn insert_pool(pool: &PgPool, nested: &EarningHistoryNestedData) -> Result<i32, Error> {
    let query = "
    INSERT INTO pools (pool_name, asset_liquidity_fees, earnings, rewards, rune_liquidity_fees, saver_earning, total_liquidity_fees_rune)
    VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id";
    let nested_id: (i32,) = sqlx::query_as(query)
        .bind(&nested.pool)
        .bind(nested.assetLiquidityFees)
        .bind(nested.earnings)
        .bind(nested.rewards)
        .bind(nested.runeLiquidityFees)
        .bind(nested.saverEarning)
        .bind(nested.totalLiquidityFeesRune)
        .fetch_one(pool)
        .await?;
    Ok(nested_id.0)
}

async fn insert_earning_interval(pool: &PgPool, interval: &EarningHistoryData, pool_ids: &[i32]) -> Result<i32, Error> {
    let query = "
    INSERT INTO earning_intervals (avg_node_count, block_rewards, bonding_earnings, earnings, end_time, liquidity_earnings, liquidity_fees, rune_price_usd, start_time, pools)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING id";
    let interval_id: (i32,) = sqlx::query_as(query)
        .bind(interval.avgNodeCount)
        .bind(interval.blockRewards)
        .bind(interval.bondingEarnings)
        .bind(interval.earnings)
        .bind(interval.endTime)
        .bind(interval.liquidityEarnings)
        .bind(interval.liquidityFees)
        .bind(interval.runePriceUsd)
        .bind(interval.startTime)
        .bind(pool_ids) // Here we bind the array of pool ids
        .fetch_one(pool)
        .await?;
    Ok(interval_id.0)
}


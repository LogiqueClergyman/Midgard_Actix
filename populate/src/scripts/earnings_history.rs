use super::utils::get_last_successful_entry_for_table;
use crate::models::earnings_history::{EarningHistoryData, EarningHistoryNestedData};
use chrono::Utc;
use reqwest::get;
use serde_json::Value;
use sqlx::{Error, PgPool};

pub async fn fetch_and_insert_data(pool: &PgPool) -> Result<(), Error> {
    let mut from_time = get_last_successful_entry_for_table(pool, "earning_history").await;
    println!("{:?}", from_time);
    let end_time = Utc::now().timestamp();

    loop {
        let url = format!(
            "https://midgard.ninerealms.com/v2/history/earnings?interval=hour&from={}&count=400",
            from_time
        );
        println!("{:?}", url);

        let response: Value = get(&url).await.unwrap().json().await.unwrap();

        for interval in response["intervals"].as_array().unwrap() {
            let mut nested_ids = Vec::new();
            for nested in interval["pools"].as_array().unwrap() {
                let nested_data: EarningHistoryNestedData =
                    serde_json::from_value(nested.clone()).unwrap();
                let nested_id = insert_pool(pool, &nested_data).await?;
                nested_ids.push(nested_id);
            }

            let interval_data = EarningHistoryData {
                avgNodeCount: interval["avgNodeCount"].as_str().unwrap().parse().unwrap(),
                blockRewards: interval["blockRewards"].as_str().unwrap().parse().unwrap(),
                bondingEarnings: interval["bondingEarnings"]
                    .as_str()
                    .unwrap()
                    .parse()
                    .unwrap(),
                earnings: interval["earnings"].as_str().unwrap().parse().unwrap(),
                endTime: interval["endTime"].as_str().unwrap().parse().unwrap(),
                liquidityEarnings: interval["liquidityEarnings"]
                    .as_str()
                    .unwrap()
                    .parse()
                    .unwrap(),
                liquidityFees: interval["liquidityFees"].as_str().unwrap().parse().unwrap(),
                runePriceUsd: interval["runePriceUSD"].as_str().unwrap().parse().unwrap(),
                startTime: interval["startTime"].as_str().unwrap().parse().unwrap(),
                pool: nested_ids,
            };
            insert_earning_interval(pool, &interval_data).await?;
        }

        let last_end_time = response["intervals"].as_array().unwrap().last().unwrap()["endTime"]
            .as_str()
            .unwrap()
            .parse::<i64>()
            .unwrap();

        if last_end_time >= end_time {
            break;
        }

        from_time = last_end_time;
    }

    Ok(())
}

async fn insert_pool(pool: &PgPool, nested: &EarningHistoryNestedData) -> Result<i32, Error> {
    let query = "
    INSERT INTO earning_history_nested (pool, assetliquidityfees, earnings, rewards, runeliquidityfees, saverearning, totalliquidityfeesrune)
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

async fn insert_earning_interval(
    pool: &PgPool,
    interval: &EarningHistoryData,
) -> Result<(), Error> {
    let query = "
    INSERT INTO earning_history (avgNodeCount, blockRewards, bondingEarnings, earnings, endTime, liquidityEarnings, liquidityFees, runePriceUsd, startTime, pools)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) ON CONFLICT (startTime, endTime) DO NOTHING RETURNING id";
    let _: (i32,) = sqlx::query_as(query)
        .bind(interval.avgNodeCount)
        .bind(interval.blockRewards)
        .bind(interval.bondingEarnings)
        .bind(interval.earnings)
        .bind(interval.endTime)
        .bind(interval.liquidityEarnings)
        .bind(interval.liquidityFees)
        .bind(interval.runePriceUsd)
        .bind(interval.startTime)
        .bind(interval.pool.clone())
        .fetch_one(pool)
        .await?;
    Ok(())
}

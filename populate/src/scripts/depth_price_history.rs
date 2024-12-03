use super::utils::get_last_successful_entry_for_table;
use crate::models::depth_price_history::DepthPriceHistory;
use bigdecimal::BigDecimal;
use chrono::Utc;
use reqwest::get;
use serde_json::Value;
use sqlx::Error;
use std::str::FromStr;

pub async fn fetch_and_insert_data(pool: &sqlx::PgPool) -> Result<(), Error> {
    let mut from_time = get_last_successful_entry_for_table(&pool, "depth_price_history").await;
    let end_time = Utc::now().timestamp();
    loop {
        let url = format!(
            "https://midgard.ninerealms.com/v2/history/depths/BTC.BTC?interval=hour&from={}&count=400", 
            from_time
        );
        println!("{:?}", url);
        let response: Value = get(&url).await.unwrap().json().await.unwrap();
        if let Some(intervals) = response["intervals"].as_array() {
            for entry in intervals {
                let asset_price_history = DepthPriceHistory {
                    assetDepth: entry["assetDepth"]
                        .as_str()
                        .unwrap()
                        .parse::<i64>()
                        .unwrap(),
                    assetPrice: entry["assetPrice"]
                        .as_str()
                        .unwrap()
                        .parse::<f64>()
                        .unwrap(),
                    assetPriceUSD: entry["assetPriceUSD"]
                        .as_str()
                        .unwrap()
                        .parse::<f64>()
                        .unwrap(),
                    liquidityUnits: entry["liquidityUnits"]
                        .as_str()
                        .unwrap()
                        .parse::<i64>()
                        .unwrap(),
                    luvi: entry["luvi"].as_str().unwrap().parse::<f64>().unwrap(),
                    membersCount: entry["membersCount"]
                        .as_str()
                        .unwrap()
                        .parse::<i32>()
                        .unwrap(),
                    runeDepth: entry["runeDepth"].as_str().unwrap().parse::<i64>().unwrap(),
                    synthSupply: entry["synthSupply"]
                        .as_str()
                        .unwrap()
                        .parse::<i64>()
                        .unwrap(),
                    synthUnits: entry["synthUnits"]
                        .as_str()
                        .unwrap()
                        .parse::<i64>()
                        .unwrap(),
                    units: entry["units"].as_str().unwrap().parse::<i64>().unwrap(),
                    startTime: entry["startTime"].as_str().unwrap().parse::<i64>().unwrap(),
                    endTime: entry["endTime"].as_str().unwrap().parse::<i64>().unwrap(),
                    id: None,
                };
                match sqlx::query!(
                    r#"
                    INSERT INTO Depth_Price_History (
                        assetDepth, assetPrice, assetPriceUSD, liquidityUnits, luvi,
                        membersCount, runeDepth, synthSupply, synthUnits, units, startTime, endTime
                    )
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
                    "#,
                    asset_price_history.assetDepth,
                    BigDecimal::from_str(&asset_price_history.assetPrice.to_string()).unwrap(),
                    BigDecimal::from_str(&asset_price_history.assetPriceUSD.to_string()).unwrap(),
                    asset_price_history.liquidityUnits,
                    BigDecimal::from_str(&asset_price_history.luvi.to_string()).unwrap(),
                    asset_price_history.membersCount,
                    asset_price_history.runeDepth,
                    asset_price_history.synthSupply,
                    asset_price_history.synthUnits,
                    asset_price_history.units,
                    asset_price_history.startTime,
                    asset_price_history.endTime
                )
                .execute(pool)
                .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        if let sqlx::Error::Database(db_err) = &e {
                            if db_err.code() == Some("23505".into()) {
                                //duplicate entry
                                continue;
                            }
                        }
                        return Err(e);
                    }
                };
            }
            let last_entry = intervals.last().unwrap();
            let last_end_time = last_entry["endTime"]
                .as_str()
                .unwrap()
                .parse::<i64>()
                .unwrap();

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

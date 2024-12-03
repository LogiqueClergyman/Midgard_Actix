mod swaps_history;
use bigdecimal::BigDecimal;
use chrono::{NaiveDate, Utc};
pub use swaps_history::SwapsHistory;
use reqwest::get;
use serde_json::Value;
use shared::{create_db_pool, run_migrations};
use sqlx::Error;
use std::str::FromStr;

static LAST_SUCCESSFUL_ENTRY: std::sync::Mutex<i64> = std::sync::Mutex::new(0);

#[tokio::main]
async fn main() {
    let pool = create_db_pool().await.unwrap();
    let pool = pool.lock().await;
    run_migrations(&pool).await.unwrap();
    let res = fetch_and_insert_data(&pool).await;
    if let Err(e) = res {
        eprintln!("Error: {:?}", e);
    }
}

async fn fetch_and_insert_data(pool: &sqlx::PgPool) -> Result<(), Error> {
    let start_date = NaiveDate::from_ymd_opt(2024, 10, 1)
        .unwrap_or_else(|| panic!("Invalid date"))
        .and_hms_opt(0, 0, 0)
        .unwrap();

    let mut from_time = start_date.timestamp();

    // Retrieve the last successful entry's endTime from the FetchState table, defaulting to 1 Oct 2024 if not found
    let last_successful_entry = get_last_successful_entry(&pool).await.unwrap_or_else(|_| {
        println!("{:?}", start_date.timestamp());
        start_date.timestamp() // Make sure to convert to DateTime<Utc>
    });
    from_time = last_successful_entry;

    // Set end time as the current time
    let end_time = Utc::now().timestamp();
    let mut count = 0;

    loop {
        let url = format!(
            "https://midgard.ninerealms.com/v2/history/swaps?interval=hour&from={}&count=100",
            from_time
        );
        println!("{:?}", url);

        let response: Value = get(&url).await.unwrap().json().await.unwrap();

        if let Some(intervals) = response["intervals"].as_array() {
            for entry in intervals {
                let swap_history = SwapsHistory {
                    id: None,
                    startTime: entry["startTime"].as_str().unwrap().parse::<i64>().unwrap(),
                    endTime: entry["endTime"].as_str().unwrap().parse::<i64>().unwrap(),
                    toAssetCount: entry["toAssetCount"]
                        .as_str()
                        .unwrap()
                        .parse::<i64>()
                        .unwrap(),
                    toRuneCount: entry["toRuneCount"]
                        .as_str()
                        .unwrap()
                        .parse::<i64>()
                        .unwrap(),
                    toTradeCount: entry["toTradeCount"]
                        .as_str()
                        .unwrap()
                        .parse::<i64>()
                        .unwrap(),
                    fromTradeCount: entry["fromTradeCount"]
                        .as_str()
                        .unwrap()
                        .parse::<i64>()
                        .unwrap(),
                    synthMintCount: entry["synthMintCount"]
                        .as_str()
                        .unwrap()
                        .parse::<i64>()
                        .unwrap(),
                    synthRedeemCount: entry["synthRedeemCount"]
                        .as_str()
                        .unwrap()
                        .parse::<i64>()
                        .unwrap(),
                    totalCount: entry["totalCount"]
                        .as_str()
                        .unwrap()
                        .parse::<i64>()
                        .unwrap(),
                    toAssetVolume: entry["toAssetVolume"]
                        .as_str()
                        .unwrap()
                        .parse::<i64>()
                        .unwrap(),
                    toRuneVolume: entry["toRuneVolume"]
                        .as_str()
                        .unwrap()
                        .parse::<i64>()
                        .unwrap(),
                    toTradeVolume: entry["toTradeVolume"]
                        .as_str()
                        .unwrap()
                        .parse::<i64>()
                        .unwrap(),
                    fromTradeVolume: entry["fromTradeVolume"]
                        .as_str()
                        .unwrap()
                        .parse::<i64>()
                        .unwrap(),
                    synthMintVolume: entry["synthMintVolume"]
                        .as_str()
                        .unwrap()
                        .parse::<i64>()
                        .unwrap(),
                    synthRedeemVolume: entry["synthRedeemVolume"]
                        .as_str()
                        .unwrap()
                        .parse::<i64>()
                        .unwrap(),
                    totalVolume: entry["totalVolume"]
                        .as_str()
                        .unwrap()
                        .parse::<i64>()
                        .unwrap(),
                    toAssetVolumeUSD: entry["toAssetVolumeUSD"]
                        .as_str()
                        .unwrap()
                        .parse::<f64>()
                        .unwrap(),
                    toRuneVolumeUSD: entry["toRuneVolumeUSD"]
                        .as_str()
                        .unwrap()
                        .parse::<f64>()
                        .unwrap(),
                    toTradeVolumeUSD: entry["toTradeVolumeUSD"]
                        .as_str()
                        .unwrap()
                        .parse::<f64>()
                        .unwrap(),
                    fromTradeVolumeUSD: entry["fromTradeVolumeUSD"]
                        .as_str()
                        .unwrap()
                        .parse::<f64>()
                        .unwrap(),
                    synthMintVolumeUSD: entry["synthMintVolumeUSD"]
                        .as_str()
                        .unwrap()
                        .parse::<f64>()
                        .unwrap(),
                    synthRedeemVolumeUSD: entry["synthRedeemVolumeUSD"]
                        .as_str()
                        .unwrap()
                        .parse::<f64>()
                        .unwrap(),
                    totalVolumeUSD: entry["totalVolumeUSD"]
                        .as_str()
                        .unwrap()
                        .parse::<f64>()
                        .unwrap(),
                    toAssetFees: entry["toAssetFees"]
                        .as_str()
                        .unwrap()
                        .parse::<i64>()
                        .unwrap(),
                    toRuneFees: entry["toRuneFees"]
                        .as_str()
                        .unwrap()
                        .parse::<i64>()
                        .unwrap(),
                    toTradeFees: entry["toTradeFees"]
                        .as_str()
                        .unwrap()
                        .parse::<i64>()
                        .unwrap(),
                    fromTradeFees: entry["fromTradeFees"]
                        .as_str()
                        .unwrap()
                        .parse::<i64>()
                        .unwrap(),
                    synthMintFees: entry["synthMintFees"]
                        .as_str()
                        .unwrap()
                        .parse::<i64>()
                        .unwrap(),
                    synthRedeemFees: entry["synthRedeemFees"]
                        .as_str()
                        .unwrap()
                        .parse::<i64>()
                        .unwrap(),
                    totalFees: entry["totalFees"].as_str().unwrap().parse::<i64>().unwrap(),
                    toAssetAverageSlip: entry["toAssetAverageSlip"]
                        .as_str()
                        .unwrap()
                        .parse::<f64>()
                        .unwrap(),
                    toRuneAverageSlip: entry["toRuneAverageSlip"]
                        .as_str()
                        .unwrap()
                        .parse::<f64>()
                        .unwrap(),
                    toTradeAverageSlip: entry["toTradeAverageSlip"]
                        .as_str()
                        .unwrap()
                        .parse::<f64>()
                        .unwrap(),
                    fromTradeAverageSlip: entry["fromTradeAverageSlip"]
                        .as_str()
                        .unwrap()
                        .parse::<f64>()
                        .unwrap(),
                    synthMintAverageSlip: entry["synthMintAverageSlip"]
                        .as_str()
                        .unwrap()
                        .parse::<f64>()
                        .unwrap(),
                    synthRedeemAverageSlip: entry["synthRedeemAverageSlip"]
                        .as_str()
                        .unwrap()
                        .parse::<f64>()
                        .unwrap(),
                    averageSlip: entry["averageSlip"]
                        .as_str()
                        .unwrap()
                        .parse::<f64>()
                        .unwrap(),
                    runePriceUSD: entry["runePriceUSD"]
                        .as_str()
                        .unwrap()
                        .parse::<f64>()
                        .unwrap(),
                };
                println!("{:?}", swap_history);
                // Insert the data into the database using SwapHistory struct
                match sqlx::query!(
                    r#"
                    INSERT INTO swap_history (
                        startTime, endTime, toAssetCount, toRuneCount, toTradeCount, fromTradeCount,
                        synthMintCount, synthRedeemCount, totalCount, toAssetVolume, toRuneVolume,
                        toTradeVolume, fromTradeVolume, synthMintVolume, synthRedeemVolume, totalVolume,
                        toAssetVolumeUSD, toRuneVolumeUSD, toTradeVolumeUSD, fromTradeVolumeUSD,
                        synthMintVolumeUSD, synthRedeemVolumeUSD, totalVolumeUSD, toAssetFees, toRuneFees,
                        toTradeFees, fromTradeFees, synthMintFees, synthRedeemFees, totalFees, 
                        toAssetAverageSlip, toRuneAverageSlip, toTradeAverageSlip, fromTradeAverageSlip, 
                        synthMintAverageSlip, synthRedeemAverageSlip, averageSlip, runePriceUSD
                    )
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, 
                            $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34, $35, $36, $37, $38)
                    "#,
                    swap_history.startTime,
                    swap_history.endTime,
                    swap_history.toAssetCount,
                    swap_history.toRuneCount,
                    swap_history.toTradeCount,
                    swap_history.fromTradeCount,
                    swap_history.synthMintCount,
                    swap_history.synthRedeemCount,
                    swap_history.totalCount,
                    swap_history.toAssetVolume,
                    swap_history.toRuneVolume,
                    swap_history.toTradeVolume,
                    swap_history.fromTradeVolume,
                    swap_history.synthMintVolume,
                    swap_history.synthRedeemVolume,
                    swap_history.totalVolume,
                    BigDecimal::from_str(&swap_history.toAssetVolumeUSD.to_string()).unwrap(),
                    BigDecimal::from_str(&swap_history.toRuneVolumeUSD.to_string()).unwrap(),
                    BigDecimal::from_str(&swap_history.toTradeVolumeUSD.to_string()).unwrap(),
                    BigDecimal::from_str(&swap_history.fromTradeVolumeUSD.to_string()).unwrap(),
                    BigDecimal::from_str(&swap_history.synthMintVolumeUSD.to_string()).unwrap(),
                    BigDecimal::from_str(&swap_history.synthRedeemVolumeUSD.to_string()).unwrap(),
                    BigDecimal::from_str(&swap_history.totalVolumeUSD.to_string()).unwrap(),
                    swap_history.toAssetFees,
                    swap_history.toRuneFees,
                    swap_history.toTradeFees,
                    swap_history.fromTradeFees,
                    swap_history.synthMintFees,
                    swap_history.synthRedeemFees,
                    swap_history.totalFees,
                    BigDecimal::from_str(&swap_history.toAssetAverageSlip.to_string()).unwrap(),
                    BigDecimal::from_str(&swap_history.toRuneAverageSlip.to_string()).unwrap(),
                    BigDecimal::from_str(&swap_history.toTradeAverageSlip.to_string()).unwrap(),
                    BigDecimal::from_str(&swap_history.fromTradeAverageSlip.to_string()).unwrap(),
                    BigDecimal::from_str(&swap_history.synthMintAverageSlip.to_string()).unwrap(),
                    BigDecimal::from_str(&swap_history.synthRedeemAverageSlip.to_string()).unwrap(),
                    BigDecimal::from_str(&swap_history.averageSlip.to_string()).unwrap(),
                    BigDecimal::from_str(&swap_history.runePriceUSD.to_string()).unwrap(),
                )
                .execute(pool)
                .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        if let sqlx::Error::Database(db_err) = &e {
                            if db_err.code() == Some("23505".into()) {
                                // Skip this entry due to duplicate key error
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

async fn get_last_successful_entry(pool: &sqlx::PgPool) -> Result<i64, Error> {
    let result = sqlx::query!(
        r#"
        SELECT last_successful_entry FROM fetch_state WHERE table_name = 'swap_history'
        "#,
    )
    .fetch_optional(pool)
    .await?;
    let default_value = NaiveDate::from_ymd_opt(2024, 10, 1)
        .unwrap_or_else(|| panic!("Invalid date"))
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .timestamp();
    let res = result
        .map(|r| r.last_successful_entry)
        .unwrap_or(default_value.into());
    if res == 0 {
        return Ok(default_value.into());
    }
    Ok(res)
}
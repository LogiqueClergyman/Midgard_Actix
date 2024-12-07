use crate::fetch::utils::get_last_successful_entry_for_table;
use crate::models::depth_price_history::DepthPriceHistory;
use chrono::Utc;
use reqwest::get;
use serde_json::Value;
pub async fn fetch_and_insert_data(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let mut from_time = get_last_successful_entry_for_table(&pool, "depth_price_history").await;
    let end_time = Utc::now().timestamp() / 3600 * 3600;
    loop {
        let url = format!(
            "https://midgard.ninerealms.com/v2/history/depths/BTC.BTC?interval=hour&from={}&count=100", 
            from_time
        );
        println!("{:?}", url);
        let response: Value = match get(&url).await {
            Ok(resp) => match resp.json().await {
                Ok(json) => json,
                Err(err) => {
                    eprintln!("JSON parse error: {}", err);
                    continue;
                }
            },
            Err(err) => {
                eprintln!("Fetch error: {}", err);
                continue;
            }
        };
        if let Some(intervals) = response["intervals"].as_array() {
            for entry in intervals {
                let depth_price_history: DepthPriceHistory = serde_json::from_value(entry.clone())?;
                sqlx::query!(
                    r#"
                        INSERT INTO Depth_Price_History (
                            asset_depth, 
                            asset_price, 
                            asset_price_usd, 
                            liquidity_units, 
                            luvi, 
                            members_count, 
                            rune_depth, 
                            synth_supply, 
                            synth_units, 
                            units, 
                            start_time, 
                            end_time
                            
                        ) 
                        VALUES (
                            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12
                        ) ON CONFLICT DO NOTHING
                        "#,
                    depth_price_history.asset_depth,
                    depth_price_history.asset_price,
                    depth_price_history.asset_price_usd,
                    depth_price_history.liquidity_units,
                    depth_price_history.luvi,
                    depth_price_history.members_count,
                    depth_price_history.rune_depth,
                    depth_price_history.synth_supply,
                    depth_price_history.synth_units,
                    depth_price_history.units,
                    depth_price_history.start_time,
                    depth_price_history.end_time
                )
                .execute(pool)
                .await?;
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

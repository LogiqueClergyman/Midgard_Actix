use crate::fetch::utils::get_last_successful_entry_for_table;
use crate::models::swaps_history::SwapsHistory;
use chrono::Utc;
use reqwest::get;
use serde_json::Value;

// use super::utils::get_last_successful_entry_for_table;

pub async fn fetch_and_insert_data(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let mut from_time = get_last_successful_entry_for_table(&pool, "swaps_history").await;
    let end_time = Utc::now().timestamp() / 3600 * 3600;
    loop {
        let url = format!(
            "https://midgard.ninerealms.com/v2/history/swaps?interval=hour&from={}&count=100",
            from_time
        );
        println!("{:?}", url);

        let response: Value = get(&url).await.unwrap().json().await?;

        if let Some(intervals) = response["intervals"].as_array() {
            for entry in intervals {
                let swap_history: SwapsHistory = serde_json::from_value(entry.clone())?;
                sqlx::query!(
                    r#"
                    INSERT INTO swap_history (
                        start_time, end_time, to_asset_count, to_rune_count, to_trade_count, from_trade_count,
                        synth_mint_count, synth_redeem_count, total_count, to_asset_volume, to_rune_volume,
                        to_trade_volume, from_trade_volume, synth_mint_volume, synth_redeem_volume, total_volume,
                        to_asset_volume_usd, to_rune_volume_usd, to_trade_volume_usd, from_trade_volume_usd,
                        synth_mint_volume_usd, synth_redeem_volume_usd, total_volume_usd, to_asset_fees, to_rune_fees,
                        to_trade_fees, from_trade_fees, synth_mint_fees, synth_redeem_fees, total_fees, 
                        to_asset_average_slip, to_rune_average_slip, to_trade_average_slip, from_trade_average_slip, 
                        synth_mint_average_slip, synth_redeem_average_slip, average_slip, rune_price_usd
                    ) 
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, 
                            $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34, $35, $36, $37, $38) ON CONFLICT DO NOTHING
                    "#,
                    swap_history.start_time,
                    swap_history.end_time,
                    swap_history.to_asset_count,
                    swap_history.to_rune_count,
                    swap_history.to_trade_count,
                    swap_history.from_trade_count,
                    swap_history.synth_mint_count,
                    swap_history.synth_redeem_count,
                    swap_history.total_count,
                    swap_history.to_asset_volume,
                    swap_history.to_rune_volume,
                    swap_history.to_trade_volume,
                    swap_history.from_trade_volume,
                    swap_history.synth_mint_volume,
                    swap_history.synth_redeem_volume,
                    swap_history.total_volume,
                    swap_history.to_asset_volume_usd,
                    swap_history.to_rune_volume_usd,
                    swap_history.to_trade_volume_usd,
                    swap_history.from_trade_volume_usd,
                    swap_history.synth_mint_volume_usd,
                    swap_history.synth_redeem_volume_usd,
                    swap_history.total_volume_usd,
                    swap_history.to_asset_fees,
                    swap_history.to_rune_fees,
                    swap_history.to_trade_fees,
                    swap_history.from_trade_fees,
                    swap_history.synth_mint_fees,
                    swap_history.synth_redeem_fees,
                    swap_history.total_fees,
                    swap_history.to_asset_average_slip,
                    swap_history.to_rune_average_slip,
                    swap_history.to_trade_average_slip,
                    swap_history.from_trade_average_slip,
                    swap_history.synth_mint_average_slip,
                    swap_history.synth_redeem_average_slip,
                    swap_history.average_slip,
                    swap_history.rune_price_usd,
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

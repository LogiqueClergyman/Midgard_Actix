use super::utils::get_last_successful_entry_for_table;
use crate::models::runepool_history::RunepoolHistory;
use chrono::Utc;
use reqwest::get;
use serde_json::Value;
pub async fn fetch_and_insert_data(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let mut from_time = get_last_successful_entry_for_table(&pool, "runepool_history").await;
    let end_time = Utc::now().timestamp() / 3600 * 3600;

    loop {
        let url = format!(
            "https://midgard.ninerealms.com/v2/history/runepool?interval=hour&from={}&count=100",
            from_time
        );
        println!("{:?}", url);
        let response: Value = get(&url).await.unwrap().json().await?;

        if let Some(intervals) = response["intervals"].as_array() {
            for entry in intervals {
                let runepool_history: RunepoolHistory = serde_json::from_value(entry.clone())?;
                sqlx::query!(
                    r#"
                    INSERT INTO runepool_history (start_time, end_time, units, count) 
                    VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING
                    "#,
                    runepool_history.start_time,
                    runepool_history.end_time,
                    runepool_history.units,
                    runepool_history.count
                )
                .execute(pool)
                .await?;
            }

            // let last_entry = match intervals.last() {
            //     Some(entry) => entry,
            //     None => break,
            // };
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

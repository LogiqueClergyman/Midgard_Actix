use super::utils::get_last_successful_entry_for_table;
use crate::models::runepool_history::RunepoolHistory;
use chrono::Utc;
use reqwest::get;
use serde_json::Value;
use sqlx::Error;
pub async fn fetch_and_insert_data(pool: &sqlx::PgPool) -> Result<(), Error> {
    let mut from_time = get_last_successful_entry_for_table(pool, "runepool_history").await;
    let end_time = Utc::now().timestamp();

    loop {
        let url = format!(
            "https://midgard.ninerealms.com/v2/history/runepool?interval=hour&from={}&count=400",
            from_time
        );
        println!("{:?}", url);
        let response: Value = get(&url).await.unwrap().json().await.unwrap();

        if let Some(intervals) = response["intervals"].as_array() {
            for entry in intervals {
                let runepool_history = RunepoolHistory {
                    id: None,
                    startTime: entry["startTime"].as_str().unwrap().parse::<i64>().unwrap(),
                    endTime: entry["endTime"].as_str().unwrap().parse::<i64>().unwrap(),
                    units: entry["units"].as_str().unwrap().parse::<i64>().unwrap(),
                    count: entry["count"].as_str().unwrap().parse::<i32>().unwrap(),
                };
                match sqlx::query!(
                    r#"
                    INSERT INTO runepool_history (startTime, endTime, units, count)
                    VALUES ($1, $2, $3, $4)
                    "#,
                    runepool_history.startTime,
                    runepool_history.endTime,
                    runepool_history.units,
                    runepool_history.count
                )
                .execute(pool)
                .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        if let sqlx::Error::Database(db_err) = &e {
                            if db_err.code() == Some("23505".into()) {
                                // duplicate key error
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

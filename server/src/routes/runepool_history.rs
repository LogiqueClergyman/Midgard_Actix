use std::sync::Arc;

// src/routes/runepool_history.rs
use crate::models::runepool_history::RunepoolHistory;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Pool, Postgres};
#[derive(Deserialize, Debug)]
pub struct QueryParams {
    pub interval: Option<String>,
    pub from: Option<i64>,
    pub to: Option<i64>,
    pub count: Option<i64>,
    pub sort_by: Option<String>,
    pub order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub units_gt: Option<i64>,
    pub count_gt: Option<i32>,
}

pub async fn get_runepool_history(
    pool: web::Data<Arc<tokio::sync::Mutex<Pool<Postgres>>>>,
    query: web::Query<QueryParams>,
) -> impl Responder {
    println!("getting_runepool_history");
    let query_str = build_query(&query);
    println!("connecting: {}", query_str);
    let pool = pool.lock().await;
    let rows = sqlx::query_as::<_, RunepoolHistory>(&query_str)
        .fetch_all(&*pool)
        .await;
    match rows {
        Ok(rows) => HttpResponse::Ok().json(rows),
        Err(err) => HttpResponse::InternalServerError().json({
            serde_json::json!({"error": "Error fetching data", "details": err.to_string()})
        }),
    }
}

fn build_query(query: &QueryParams) -> String {
    println!("{:?}", query);
    let mut where_clauses = vec![];

    if let Some(from) = query.from {
        where_clauses.push(format!("starttime >= {}", from));
    }

    if let Some(to) = query.to {
        where_clauses.push(format!("endtime <= {}", to));
    }

    if let Some(units_gt) = query.units_gt {
        where_clauses.push(format!("units > {}", units_gt));
    }

    if let Some(count_gt) = query.count_gt {
        where_clauses.push(format!("count > {}", count_gt));
    }

    let where_sql = if where_clauses.is_empty() {
        "TRUE".to_string()
    } else {
        where_clauses.join(" AND ")
    };
    println!("{}", where_sql);
    let sort_by = query
        .sort_by
        .clone()
        .unwrap_or_else(|| "starttime".to_string());
    let order = query.order.clone().unwrap_or_else(|| "asc".to_string());
    let order_sql = if order == "desc" { "DESC" } else { "ASC" };

    if let Some(interval) = &query.interval {
        let interval_seconds = match interval.as_str() {
            "hour" => 3600,
            "day" => 86400,
            "week" => 604800,
            "month" => 2592000, // Approximation for a 30-day month
            "year" => 31536000,
            _ => 3600, // Default to hourly if an invalid interval is provided
        };

        let count = query.count.unwrap_or(1).min(400);
        let (limit, offset) = paginate(query.page, query.limit);
        if interval == "hour" {
            // Directly query hourly data
            format!(
                r#"
                    SELECT starttime, endtime, units, count
                    FROM runepool_history
                    WHERE {}
                    ORDER BY starttime {}
                    LIMIT {}
                    "#,
                where_sql, order_sql, count
            )
        } else {
            // Handle larger intervals (day, week, etc.)
            format!(
                r#"
                    WITH grouped_data AS (
                        SELECT
                            (starttime / {interval_seconds}) * {interval_seconds} AS bracket_start,
                            starttime, endtime, units, count,
                            ROW_NUMBER() OVER (
                                PARTITION BY (starttime / {interval_seconds})
                                ORDER BY starttime DESC
                            ) AS rank
                        FROM runepool_history
                        WHERE {where_sql}
                    )
                    SELECT
                        MIN(bracket_start) AS starttime, -- First hour of the bracket
                        MAX(starttime) AS endtime,      -- Last hour of the bracket
                        units,
                        count
                    FROM grouped_data
                    WHERE rank = 1
                    GROUP BY bracket_start, units, count
                    ORDER BY starttime {order_sql}
                    LIMIT {limit} OFFSET {offset}
                    "#,
                interval_seconds = interval_seconds,
                where_sql = where_sql,
                order_sql = order_sql,
                limit = limit,
                offset = offset
            )
        }
    } else {
        // Fallback for no interval specified
        let (limit, offset) = paginate(query.page, query.limit);

        format!(
            r#"
                SELECT starttime, endtime, units, count
                FROM runepool_history
                WHERE {}
                ORDER BY {} {}
                LIMIT {} OFFSET {}
                "#,
            where_sql, sort_by, order_sql, limit, offset
        )
    }
}
fn paginate(page: Option<i64>, limit: Option<i64>) -> (i64, i64) {
    let page = page.unwrap_or(1);
    let limit = limit.unwrap_or(100);
    let offset = (page - 1) * limit;
    (limit, offset)
}

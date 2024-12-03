use super::utils::{paginate, add_condition};
use crate::models::runepool_history::{QueryParams, RunepoolHistory};
use actix_web::{web, HttpResponse, Responder};
use sqlx::{Pool, Postgres};
use std::sync::Arc;

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

    // Time range filters
    if let Some(from) = query.from {
        where_clauses.push(format!("starttime >= {}", from));
    }

    if let Some(to) = query.to {
        where_clauses.push(format!("endtime <= {}", to));
    }

    // Filters for `units`
    if let Some(units_gt) = query.units_gt {
        where_clauses.push(format!("units > {}", units_gt));
    }

    if let Some(units_lt) = query.units_lt {
        where_clauses.push(format!("units < {}", units_lt));
    }

    if let Some(units_eq) = query.units_eq {
        where_clauses.push(format!("units = {}", units_eq));
    }

    // Filters for `count`
    if let Some(count_gt) = query.count_gt {
        where_clauses.push(format!("count > {}", count_gt));
    }

    if let Some(count_lt) = query.count_lt {
        where_clauses.push(format!("count < {}", count_lt));
    }

    if let Some(count_eq) = query.count_eq {
        where_clauses.push(format!("count = {}", count_eq));
    }

    // Combining all WHERE clauses
    let where_sql = if where_clauses.is_empty() {
        "TRUE".to_string()
    } else {
        where_clauses.join(" AND ")
    };

    println!("WHERE clause: {}", where_sql);

    // Sorting and ordering logic
    let sort_by = query
        .sort_by
        .clone()
        .unwrap_or_else(|| "starttime".to_string());
    let order = query.order.clone().unwrap_or_else(|| "asc".to_string());
    let order_sql = if order == "desc" { "DESC" } else { "ASC" };

    let hard_limit = query.count.unwrap_or(400).min(400);
    let (pagination_limit, offset) = paginate(query.page, query.limit, query.count);
    let effective_limit = hard_limit.min(pagination_limit);
    // Handling interval
    if let Some(interval) = &query.interval {
        let interval_sql = match interval.as_str() {
            "hour" => "hour",
            "day" => "day",
            "week" => "week",
            "month" => "month",
            "quarter" => "quarter",
            "year" => "year",
            _ => "hour", // Default to hourly if an invalid interval is provided
        };

        if interval == "hour" {
            // Directly query hourly data
            format!(
                r#"
                SELECT starttime, endtime, units, count
                FROM runepool_history
                WHERE {}
                ORDER BY starttime {}
                LIMIT {}
                OFFSET {}
                "#,
                where_sql, order_sql, effective_limit, offset
            )
        } else {
            // Aggregate for larger intervals
            format!(
                r#"
        WITH grouped_data AS (
            SELECT
                EXTRACT(EPOCH FROM date_trunc('{}', to_timestamp(starttime)))::BIGINT AS bracket_start,
                starttime, endtime, units, count,
                ROW_NUMBER() OVER (
                    PARTITION BY date_trunc('{}', to_timestamp(starttime))
                    ORDER BY starttime DESC
                ) AS rank
            FROM runepool_history
            WHERE {}
        )
        SELECT
            MIN(bracket_start) AS starttime,  -- First hour of the bracket
            MAX(starttime) AS endtime,       -- Last hour of the bracket
            units,
            count
        FROM grouped_data
        WHERE rank = 1
        GROUP BY bracket_start, units, count
        ORDER BY starttime {}
        LIMIT {}
        OFFSET {}
        "#,
                interval_sql, interval_sql, where_sql, order_sql, effective_limit, offset
            )
        }
    } else {
        // No interval specified, default behavior
        format!(
            r#"
            SELECT starttime, endtime, units, count
            FROM runepool_history
            WHERE {}
            ORDER BY {} {}
            LIMIT {} OFFSET {}
            "#,
            where_sql, sort_by, order_sql, effective_limit, offset
        )
    }
}

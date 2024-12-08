use super::utils::{add_condition, paginate};
use crate::models::runepool_history::{QueryParams, RunepoolHistory};
use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;

pub async fn get_runepool_history(
    pool: web::Data<Arc<sqlx::PgPool>>,
    query: web::Query<QueryParams>,
) -> impl Responder {
    let query_str = build_query(&query);
    println!("connecting: {}", query_str);
    let rows = sqlx::query_as::<_, RunepoolHistory>(&query_str)
        .fetch_all(&***pool)
        .await;
    match rows {
        Ok(rows) => HttpResponse::Ok().json(rows),
        Err(err) => HttpResponse::InternalServerError().json({
            serde_json::json!({"error": "Error fetching data", "details": err.to_string()})
        }),
    }
}

fn build_query(query: &QueryParams) -> String {
    let mut where_clauses: Vec<String> = vec![];
    add_condition(&mut where_clauses, "start_time", &query.from, ">");
    add_condition(&mut where_clauses, "end_time", &query.to, "<");
    add_condition(&mut where_clauses, "units", &query.units_gt, ">");
    add_condition(&mut where_clauses, "units", &query.units_lt, "<");
    add_condition(&mut where_clauses, "units", &query.units_eq, "=");
    // Combining all WHERE clauses
    let where_sql = if where_clauses.is_empty() {
        "TRUE".to_string()
    } else {
        where_clauses.join(" AND ")
    };

    // Sorting and ordering logic
    let sort_by = query
        .sort_by
        .clone()
        .unwrap_or_else(|| "start_time".to_string());
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
                SELECT *
                FROM runepool_history
                WHERE {}
                ORDER BY {} {}
                LIMIT {}
                OFFSET {}
                "#,
                where_sql, sort_by, order_sql, effective_limit, offset
            )
        } else {
            // Aggregate for larger intervals
            format!(
                r#"
        WITH grouped_data AS (
            SELECT
                EXTRACT(EPOCH FROM date_trunc('{}', to_timestamp(start_time)))::BIGINT AS bracket_start,
                start_time, end_time, units, count,
                ROW_NUMBER() OVER (
                    PARTITION BY date_trunc('{}', to_timestamp(start_time))
                    ORDER BY start_time DESC
                ) AS rank
            FROM runepool_history
            WHERE {}
        )
        SELECT
            MIN(bracket_start) AS start_time,  -- First hour of the bracket
            MAX(start_time) AS end_time,       -- Last hour of the bracket
            units,
            count
        FROM grouped_data
        WHERE rank = 1
        GROUP BY bracket_start, units, count
        ORDER BY start_time {}
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
            SELECT start_time, end_time, units, count
            FROM runepool_history
            WHERE {}
            ORDER BY {} {}
            LIMIT {} OFFSET {}
            "#,
            where_sql, sort_by, order_sql, effective_limit, offset
        )
    }
}

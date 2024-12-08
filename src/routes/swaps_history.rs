use super::utils::{add_condition, paginate};
use crate::models::swaps_history::{SwapQueryParams, SwapsHistory};
use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;

pub async fn get_swap_history(
    pool: web::Data<Arc<sqlx::PgPool>>,
    query: web::Query<SwapQueryParams>,
) -> impl Responder {
    let query_str = build_swap_query(&query);
    println!("Generated query: {}", query_str);
    let rows = sqlx::query_as::<_, SwapsHistory>(&query_str)
        .fetch_all(&***pool)
        .await;
    match rows {
        Ok(rows) => HttpResponse::Ok().json(rows),
        Err(err) => HttpResponse::InternalServerError().json({
            serde_json::json!({"error": "Error fetching data", "details": err.to_string()})
        }),
    }
}

fn build_swap_query(query: &SwapQueryParams) -> String {
    let mut where_clauses = vec![];

    // Dynamic filters
    add_condition(&mut where_clauses, "start_time", &query.from, ">");
    add_condition(&mut where_clauses, "end_time", &query.to, "<");
    add_condition(
        &mut where_clauses,
        "to_asset_volume",
        &query.to_asset_volume_gt,
        ">",
    );
    add_condition(
        &mut where_clauses,
        "to_asset_volume",
        &query.to_asset_volume_lt,
        "<",
    );
    add_condition(
        &mut where_clauses,
        "to_asset_volume",
        &query.to_asset_volume_eq,
        "=",
    );

    add_condition(
        &mut where_clauses,
        "total_volume_usd",
        &query.total_volume_usd_gt,
        ">",
    );
    add_condition(
        &mut where_clauses,
        "total_volume_usd",
        &query.total_volume_usd_lt,
        "<",
    );
    add_condition(
        &mut where_clauses,
        "total_volume_usd",
        &query.total_volume_usd_eq,
        "=",
    );

    add_condition(
        &mut where_clauses,
        "rune_price_usd",
        &query.rune_price_usd_gt,
        ">",
    );
    add_condition(
        &mut where_clauses,
        "rune_price_usd",
        &query.rune_price_usd_lt,
        "<",
    );
    add_condition(
        &mut where_clauses,
        "rune_price_usd",
        &query.rune_price_usd_eq,
        "=",
    );

    let where_sql = if where_clauses.is_empty() {
        "TRUE".to_string()
    } else {
        where_clauses.join(" AND ")
    };

    // Sorting and pagination
    let sort_by = query
        .sort_by
        .clone()
        .unwrap_or_else(|| "start_time".to_string());
    let order = query.order.clone().unwrap_or_else(|| "asc".to_string());
    let order_sql = if order == "desc" { "DESC" } else { "ASC" };
    let hard_limit = query.count.unwrap_or(400).min(400);
    let (pagination_limit, offset) = paginate(query.page, query.limit, query.count);
    let effective_limit = hard_limit.min(pagination_limit);

    let interval = match query.interval.as_deref() {
        Some("hour") => "hour",
        Some("day") => "day",     // Truncate to day
        Some("week") => "week",   // Truncate to week
        Some("month") => "month", // Truncate to month
        Some("year") => "year",   // Truncate to year
        _ => "hour",              // Default to hourly
    };

    // Build the SQL query with DATE_TRUNC for grouping
    format!(
        r#"
    WITH first_and_last_hours AS (
        SELECT 
            DATE_TRUNC('{interval}', to_timestamp(start_time)) AS bracket_start,
            MIN(start_time) AS first_hour_start_time,
            MAX(start_time) AS last_hour_start_time
        FROM swap_history
        WHERE {where_sql}
        GROUP BY DATE_TRUNC('{interval}', to_timestamp(start_time))
    )
    SELECT 
        fh.bracket_start,
        fh.first_hour_start_time AS start_time,
        last_sh.*
    FROM first_and_last_hours fh
    JOIN swap_history last_sh ON last_sh.start_time = fh.last_hour_start_time
    WHERE {where_sql}
    ORDER BY fh.bracket_start {order_sql}
    LIMIT {limit} OFFSET {offset}
"#,
        interval = interval,
        where_sql = where_sql,   // Dynamic WHERE conditions
        order_sql = order_sql,   // Sorting order (asc/desc)
        limit = effective_limit, // Pagination limit
        offset = offset          // Pagination offset
    )
}

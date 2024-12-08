use super::utils::{add_condition, paginate};
use crate::models::depth_price_history::{DepthPriceHistory, QueryParams};
use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;

pub async fn get_depth_price_history(
    pool: web::Data<Arc<sqlx::PgPool>>,
    query: web::Query<QueryParams>,
) -> impl Responder {
    let query_str = build_query(&query);
    println!("connecting: {}", query_str);

    let rows = sqlx::query_as::<_, DepthPriceHistory>(&query_str)
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
    // Start building the where clause
    let mut where_clauses = vec![];
    // Add conditions for each field dynamically
    add_field_conditions(&mut where_clauses, query);
    
    // Combine all where clauses into one string
    let where_sql = if where_clauses.is_empty() {
        "TRUE".to_string()
    } else {
        where_clauses.join(" AND ")
    };
    
    // Handle pagination and sorting logic
    let order = query.order.clone().unwrap_or_else(|| "asc".to_string());
    let order_sql = if order == "desc" { "DESC" } else { "ASC" };
    let hard_limit = query.count.unwrap_or(400).min(400);
    let (pagination_limit, offset) = paginate(query.page, query.limit, query.count);
    let effective_limit = hard_limit.min(pagination_limit);
    let sort_by = query
        .sort_by
        .clone()
        .unwrap_or_else(|| "start_time".to_string());
    
    // Handle interval-based query generation
    if let Some(interval) = &query.interval {
        // Determine the interval in seconds
        let (interval_seconds, adjustment) = match interval.as_str() {
            "hour" => (3600, 0),
            "day" => (86400, 86400 - 3600),
            "week" => (604800, 604800 - 3600),
            "month" => (2592000, 2592000 - 3600),
            "year" => (31536000, 31536000 - 3600),
            _ => (3600, 0), // default to hour
        };
        
        format!(
            r#"
            WITH interval_data AS (
                SELECT 
                    (start_time / {interval_seconds}) * {interval_seconds} AS interval_start,
                    *,
                    ROW_NUMBER() OVER (
                        PARTITION BY (start_time / {interval_seconds}) 
                        ORDER BY start_time DESC
                    ) AS rank
                FROM depth_price_history
                WHERE {where_sql}
            )
            SELECT 
                interval_start - {adjustment} AS start_time,
                *
            FROM interval_data
            WHERE 
                rank = 1 AND 
                start_time >= interval_start AND
                start_time < interval_start + {interval_seconds}
            ORDER BY interval_start {order_sql}
            LIMIT {limit} OFFSET {offset}
            "#,
            interval_seconds = interval_seconds,
            adjustment = adjustment,
            where_sql = where_sql,
            order_sql = order_sql,
            limit = effective_limit,
            offset = offset
        )
    } else {
        // Default query without interval grouping
        format!(
            r#"
            SELECT * FROM depth_price_history
            WHERE {where_sql}
            ORDER BY {sort_by} {order_sql}
            LIMIT {limit} OFFSET {offset}
            "#,
            where_sql = where_sql,
            sort_by = sort_by,
            order_sql = order_sql,
            limit = effective_limit,
            offset = offset
        )
    }
}
fn add_field_conditions(where_clauses: &mut Vec<String>, query: &QueryParams) {
    add_condition(where_clauses, "start_time", &query.from, ">");
    add_condition(where_clauses, "end_time", &query.to, "<");
    add_condition(where_clauses, "asset_price", &query.assetprice_gt, ">");
    add_condition(where_clauses, "asset_price", &query.assetprice_lt, "<");
    add_condition(where_clauses, "asset_price", &query.assetprice_eq, "=");
    add_condition(
        where_clauses,
        "liquidity_units",
        &query.liquidityunits_gt,
        ">",
    );
    add_condition(
        where_clauses,
        "liquidity_units",
        &query.liquidityunits_lt,
        "<",
    );
    add_condition(
        where_clauses,
        "liquidity_units",
        &query.liquidityunits_eq,
        "=",
    );
    add_condition(where_clauses, "luvi", &query.luvi_gt, ">");
    add_condition(where_clauses, "luvi", &query.luvi_lt, "<");
    add_condition(where_clauses, "luvi", &query.luvi_eq, "=");
    add_condition(where_clauses, "members_count", &query.memberscount_gt, ">");
    add_condition(where_clauses, "members_count", &query.memberscount_lt, "<");
    add_condition(where_clauses, "members_count", &query.memberscount_eq, "=");
    add_condition(where_clauses, "rune_depth", &query.runedepth_gt, ">");
    add_condition(where_clauses, "rune_depth", &query.runedepth_lt, "<");
    add_condition(where_clauses, "rune_depth", &query.runedepth_eq, "=");
    add_condition(where_clauses, "synth_supply", &query.synthsupply_gt, ">");
    add_condition(where_clauses, "synth_supply", &query.synthsupply_lt, "<");
    add_condition(where_clauses, "synth_supply", &query.synthsupply_eq, "=");
    add_condition(where_clauses, "synth_units", &query.synthunits_gt, ">");
    add_condition(where_clauses, "synth_units", &query.synthunits_lt, "<");
    add_condition(where_clauses, "synth_units", &query.synthunits_eq, "=");
    add_condition(where_clauses, "units", &query.units_gt, ">");
    add_condition(where_clauses, "units", &query.units_lt, "<");
    add_condition(where_clauses, "units", &query.units_eq, "=");

}

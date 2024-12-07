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

    let interval_seconds = match query.interval.as_deref() {
        Some("day") => 86400,     // 1 day = 86400 seconds (24 * 60 * 60)
        Some("week") => 604800,   // 1 week = 604800 seconds (7 * 24 * 60 * 60)
        Some("month") => 2592000, // 1 month ≈ 2592000 seconds (30 days)
        Some("year") => 31536000, // 1 year = 31536000 seconds
        _ => 3600,                // Default to hourly (1 hour = 60 * 60)
    };

    // Build the SQL query
    format!(
        r#"
        WITH grouped_data AS (
            SELECT
                -- Calculate the start of the time bracket based on the interval
                CASE
                    WHEN {interval_seconds} = 86400 THEN (start_time / 86400) * 86400  -- Day
                    WHEN {interval_seconds} = 604800 THEN (start_time / 604800) * 604800  -- Week
                    WHEN {interval_seconds} = 2592000 THEN (start_time / 2592000) * 2592000  -- Month
                    WHEN {interval_seconds} = 31536000 THEN (start_time / 31536000) * 31536000  -- Year
                    ELSE start_time  -- Default to hourly (no change)
                END AS bracket_start,
                
                -- Select all columns from swap_history
                sh.*,  
                
                -- Row numbering to partition by bracket_start
                ROW_NUMBER() OVER (
                    PARTITION BY 
                        CASE
                            WHEN {interval_seconds} = 86400 THEN (start_time / 86400) * 86400  -- Day
                            WHEN {interval_seconds} = 604800 THEN (start_time / 604800) * 604800  -- Week
                            WHEN {interval_seconds} = 2592000 THEN (start_time / 2592000) * 2592000  -- Month
                            WHEN {interval_seconds} = 31536000 THEN (start_time / 31536000) * 31536000  -- Year
                            ELSE start_time  -- Default to hourly (no change)
                        END
                    ORDER BY {sort_by} DESC  -- Sort by the requested column
                ) AS rank
            FROM swap_history sh
            WHERE {where_sql}  -- Apply dynamic WHERE conditions
        )
        
        -- Select the aggregated results
        SELECT
            MIN(bracket_start) AS start_time,  -- Start time for the time bracket
            MAX(end_time) AS end_time,        -- End time for the time bracket
            avg_node_count,                    -- Average node count
            block_rewards,                     -- Block rewards
            bonding_earnings,                  -- Bonding earnings
            earnings,                          -- Earnings
            liquidity_earnings,                -- Liquidity earnings
            liquidity_fees,                    -- Liquidity fees
            rune_price_usd,                    -- Rune price in USD
            nested_pools AS pool               -- Aggregated pool data (from subquery)
        FROM grouped_data
        WHERE rank = 1  -- Ensure we select the most recent record within the group
        GROUP BY 
            avg_node_count, block_rewards, bonding_earnings, earnings, 
            liquidity_earnings, liquidity_fees, rune_price_usd, nested_pools
        ORDER BY {sort_by} {order_sql}  -- Sorting based on user input
        LIMIT {limit} OFFSET {offset}  -- Pagination: limit and offset for results
        "#,
        interval_seconds = interval_seconds,
        where_sql = where_sql,   // Dynamic WHERE conditions
        sort_by = sort_by,       // Sort field (e.g., "start_time")
        order_sql = order_sql,   // Sorting order (asc/desc)
        limit = effective_limit, // Pagination limit
        offset = offset          // Pagination offset
    )
}

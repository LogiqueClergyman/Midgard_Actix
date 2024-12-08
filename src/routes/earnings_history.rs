use super::utils::{add_condition, paginate};
use crate::models::earnings_history::{EarningHistory, EarningHistoryQueryParams};
use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;

pub async fn get_earning_history(
    pool: web::Data<Arc<sqlx::PgPool>>,
    query: web::Query<EarningHistoryQueryParams>,
) -> impl Responder {
    let query_str = build_earning_history_query(&query);
    println!("Generated query: {}", query_str);
    let rows = sqlx::query_as::<_, EarningHistory>(&query_str)
        .fetch_all(&***pool)
        .await;

    match rows {
        Ok(deserialized_rows) => HttpResponse::Ok().json(deserialized_rows),
        Err(err) => HttpResponse::InternalServerError().json({
            serde_json::json!({"error": "Error fetching data", "details": err.to_string()})
        }),
    }
}

fn build_earning_history_query(query: &EarningHistoryQueryParams) -> String {
    let mut where_clauses = vec![];

    // Dynamic filters for earning_history fields
    add_condition(&mut where_clauses, "start_time", &query.from, ">");
    add_condition(&mut where_clauses, "end_time", &query.to, "<");
    add_condition(
        &mut where_clauses,
        "avg_node_count",
        &query.avg_node_count_gt,
        ">",
    );
    add_condition(
        &mut where_clauses,
        "avg_node_count",
        &query.avg_node_count_lt,
        "<",
    );
    add_condition(
        &mut where_clauses,
        "block_rewards",
        &query.block_rewards_gt,
        ">",
    );
    add_condition(
        &mut where_clauses,
        "block_rewards",
        &query.block_rewards_lt,
        "<",
    );
    add_condition(&mut where_clauses, "earnings", &query.earnings_gt, ">");
    add_condition(&mut where_clauses, "earnings", &query.earnings_lt, "<");
    add_condition(
        &mut where_clauses,
        "liquidityearnings",
        &query.liquidity_earnings_gt,
        ">",
    );
    add_condition(
        &mut where_clauses,
        "liquidity_earnings",
        &query.liquidity_earnings_lt,
        "<",
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
        Some("day") => 86400,     // 24 * 60 * 60
        Some("week") => 604800,   // 7 * 24 * 60 * 60
        Some("month") => 2592000, // Approximate 30 days
        _ => 3600,                // Default to hourly (1 hour = 60 * 60)
    };
    // Final SQL Query
    format!(
        r#"
    WITH grouped_data AS (
        SELECT
            -- Calculate the start of the time bracket (day/week/month)
            CASE
                WHEN {interval_seconds} = 86400 THEN (start_time / 86400) * 86400 -- Day
                WHEN {interval_seconds} = 604800 THEN (start_time / 604800) * 604800 -- Week
                WHEN {interval_seconds} = 2592000 THEN (start_time / 2592000) * 2592000 -- Month
                ELSE start_time -- Default to hour (no change)
            END AS bracket_start,
            eh.*,
            ROW_NUMBER() OVER (
                PARTITION BY CASE
                    WHEN {interval_seconds} = 86400 THEN (start_time / 86400) * 86400 -- Day
                    WHEN {interval_seconds} = 604800 THEN (start_time / 604800) * 604800 -- Week
                    WHEN {interval_seconds} = 2592000 THEN (start_time / 2592000) * 2592000 -- Month
                    ELSE start_time -- Default to hour (no change)
                END
                ORDER BY {sort_by} DESC
            ) AS rank,
            (
                -- Join with earning_history_nested table to fetch related pool data
                SELECT array_agg(
                    jsonb_build_object(
                        'pool', en.pool,
                        'asset_liquidity_fees', en.asset_liquidity_fees,
                        'earnings', en.earnings,
                        'rewards', en.rewards,
                        'rune_liquidity_fees', en.rune_liquidity_fees,
                        'saver_earning', en.saver_earning,
                        'total_liquidity_fees_rune', en.total_liquidity_fees_rune
                    )
                )
                FROM earning_history_nested en
                WHERE en.earning_history_id = eh.id -- Join with earning_history_id
            ) AS nested_pools
        FROM earning_history eh
        WHERE {where_sql}
    )
    SELECT
        MIN(bracket_start) AS start_time,
        MAX(end_time) AS end_time,
        avg_node_count,
        block_rewards,
        bonding_earnings,
        earnings,
        liquidity_earnings,
        liquidity_fees,
        rune_price_usd,
        nested_pools AS pool
    FROM grouped_data
    WHERE rank = 1
    GROUP BY 
        avg_node_count, block_rewards, bonding_earnings, earnings, 
        liquidity_earnings, liquidity_fees, rune_price_usd, nested_pools
    ORDER BY {sort_by} {order_sql}
    LIMIT {limit} OFFSET {offset}
    "#,
        interval_seconds = interval_seconds,
        where_sql = where_sql,
        sort_by = sort_by,
        order_sql = order_sql,
        limit = effective_limit,
        offset = offset
    )
}

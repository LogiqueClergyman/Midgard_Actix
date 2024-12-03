use super::utils::{add_condition, paginate};
use crate::models::earnings_history::{EarningHistoryQueryParams, EarningHistoryResponse};
use actix_web::{web, HttpResponse, Responder};
use sqlx::{Pool, Postgres};
use std::sync::Arc;

pub async fn get_earning_history(
    pool: web::Data<Arc<tokio::sync::Mutex<Pool<Postgres>>>>,
    query: web::Query<EarningHistoryQueryParams>,
) -> impl Responder {
    println!("Getting earning history");
    let query_str = build_earning_history_query(&query);
    println!("Generated query: {}", query_str);

    let pool = pool.lock().await;
    let rows = sqlx::query_as::<_, EarningHistoryResponse>(&query_str)
        .fetch_all(&*pool)
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

    // Time range filters
    if let Some(from) = query.from {
        where_clauses.push(format!("starttime >= {}", from));
    }
    if let Some(to) = query.to {
        where_clauses.push(format!("endtime <= {}", to));
    }

    // Dynamic filters for earning_history fields
    add_condition(
        &mut where_clauses,
        "avgnodecount",
        &query.avg_node_count_gt,
        ">",
    );
    add_condition(
        &mut where_clauses,
        "avgnodecount",
        &query.avg_node_count_lt,
        "<",
    );
    add_condition(
        &mut where_clauses,
        "blockrewards",
        &query.block_rewards_gt,
        ">",
    );
    add_condition(
        &mut where_clauses,
        "blockrewards",
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
        "liquidityearnings",
        &query.liquidity_earnings_lt,
        "<",
    );
    add_condition(
        &mut where_clauses,
        "runepriceusd",
        &query.rune_price_usd_gt,
        ">",
    );
    add_condition(
        &mut where_clauses,
        "runepriceusd",
        &query.rune_price_usd_lt,
        "<",
    );
    add_condition(
        &mut where_clauses,
        "runepriceusd",
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
        .unwrap_or_else(|| "starttime".to_string());
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
            (starttime / {interval_seconds}) * {interval_seconds} AS bracket_start,
            eh.avgnodecount ::FLOAT8 AS avgnodecount,
            eh.blockrewards,
            eh.bondingearnings,
            eh.earnings,
            eh.endtime,
            eh.liquidityearnings,
            eh.liquidityfees,
            eh.runepriceusd,
            eh.starttime,
            eh.pools,
            ROW_NUMBER() OVER (
                PARTITION BY (starttime / {interval_seconds})
                ORDER BY {sort_by} DESC
            ) AS rank,
            (
                SELECT array_agg(
                    jsonb_build_object(
                        'pool', en.pool,
                        'asset_liquidity_fees', en.assetliquidityfees,
                        'earnings', en.earnings,
                        'rewards', en.rewards,
                        'rune_liquidity_fees', en.runeliquidityfees,
                        'saver_earning', en.saverearning,
                        'total_liquidity_fees_rune', en.totalliquidityfeesrune
                    )
                )
                FROM earning_history_nested en
                WHERE en.id = ANY(eh.pools)
            ) AS nested_pools
        FROM earning_history eh
        WHERE {where_sql}
    )
    SELECT
        MIN(bracket_start) AS starttime,
        MAX(starttime) AS endtime,
        MIN(starttime) AS first_starttime,
        MAX(endtime) AS last_endtime,
        avgnodecount,
        blockrewards,
        bondingearnings,
        earnings,
        liquidityearnings,
        liquidityfees,
        runepriceusd,
        nested_pools AS pools
    FROM grouped_data
    WHERE rank = 1
    GROUP BY 
        avgnodecount, blockrewards, bondingearnings, earnings, 
        liquidityearnings, liquidityfees, runepriceusd, nested_pools
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

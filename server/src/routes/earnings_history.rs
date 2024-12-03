// use crate::models::earnings_history::EarningHistoryResponse;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{
    prelude::{FromRow, Type},
    Pool, Postgres,
};
use std::sync::Arc;
#[derive(Serialize, Deserialize, FromRow, Debug, Type)]
pub struct EarningHistoryResponse {
    pub avgnodecount: f64,
    pub blockrewards: i64,
    pub bondingearnings: i64,
    pub earnings: i64,
    pub endtime: i64,
    pub liquidityearnings: i64,
    pub liquidityfees: i64,
    pub runepriceusd: f64,
    pub starttime: i64,
    pub pools: Vec<Value>,
}

#[derive(Serialize, Deserialize, FromRow, Debug, Type)]
#[sqlx(type_name = "earning_history_nested")]
pub struct EarningHistoryNestedResponse {
    pub pool: String,
    pub asset_liquidity_fees: i64,
    pub earnings: i64,
    pub rewards: i64,
    pub rune_liquidity_fees: i64,
    pub saver_earning: i64,
    pub total_liquidity_fees_rune: i64,
}

#[derive(Deserialize, Debug)]
pub struct EarningHistoryQueryParams {
    pub interval: Option<String>,
    pub from: Option<i64>,
    pub to: Option<i64>,
    pub sort_by: Option<String>,
    pub order: Option<String>,
    pub page: Option<i32>,
    pub limit: Option<i32>,
    pub count: Option<i32>,

    // Dynamic filters for earning_history fields
    pub avg_node_count_gt: Option<f64>,
    pub avg_node_count_lt: Option<f64>,
    pub block_rewards_gt: Option<i64>,
    pub block_rewards_lt: Option<i64>,

    pub earnings_gt: Option<i64>,
    pub earnings_lt: Option<i64>,
    pub liquidity_earnings_gt: Option<i64>,
    pub liquidity_earnings_lt: Option<i64>,

    pub rune_price_usd_gt: Option<f64>,
    pub rune_price_usd_lt: Option<f64>,
    pub rune_price_usd_eq: Option<f64>,
}

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
    let (pagination_limit, offset) = paginate(query.page, query.limit);
    let effective_limit = hard_limit.min(pagination_limit);

    // Final SQL Query
    format!(
        r#"
        SELECT 
            eh.avgnodecount ::FLOAT8 AS avgnodecount,
            eh.blockrewards,
            eh.bondingearnings,
            eh.earnings,
            eh.endtime,
            eh.liquidityearnings,
            eh.liquidityfees,
            eh.runepriceusd,
            eh.starttime,
            (
                SELECT array_agg(
                    jsonb_build_object(
                        'pool', en.pool,
                        'assetLiquidityFees', en.assetliquidityfees,
                        'earnings', en.earnings,
                        'rewards', en.rewards,
                        'runeLiquidityFees', en.runeliquidityfees,
                        'saverEarning', en.saverearning,
                        'totalLiquidityFeesRune', en.totalliquidityfeesrune
                    )
                )
                FROM earning_history_nested en
                WHERE en.id = ANY(eh.pools)
            ) AS pools
        FROM earning_history eh
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

fn add_condition<T: std::fmt::Display>(
    where_clauses: &mut Vec<String>,
    column: &str,
    value: &Option<T>,
    operator: &str,
) {
    if let Some(val) = value {
        where_clauses.push(format!("{} {} {}", column, operator, val));
    }
}

fn paginate(page: Option<i32>, limit: Option<i32>) -> (i32, i32) {
    let limit = limit.unwrap_or(10).max(1).min(100);
    let offset = page.unwrap_or(1).saturating_sub(1) * limit;
    (limit, offset)
}

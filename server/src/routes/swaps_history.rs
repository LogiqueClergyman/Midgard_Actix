use crate::models::swap_history::SwapHistory;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use std::sync::Arc;

#[derive(Deserialize, Debug)]
pub struct SwapQueryParams {
    pub interval: Option<String>,
    pub from: Option<i64>,
    pub to: Option<i64>,
    pub sort_by: Option<String>,
    pub order: Option<String>,
    pub page: Option<i32>,
    pub limit: Option<i32>,
    pub count: Option<i32>,

    // Dynamic filters for swap_history fields
    pub to_asset_volume_gt: Option<i64>,
    pub to_asset_volume_lt: Option<i64>,
    pub to_asset_volume_eq: Option<i64>,

    pub total_volume_usd_gt: Option<f64>,
    pub total_volume_usd_lt: Option<f64>,
    pub total_volume_usd_eq: Option<f64>,

    pub rune_price_usd_gt: Option<f64>,
    pub rune_price_usd_lt: Option<f64>,
    pub rune_price_usd_eq: Option<f64>,
}

pub async fn get_swap_history(
    pool: web::Data<Arc<tokio::sync::Mutex<Pool<Postgres>>>>,
    query: web::Query<SwapQueryParams>,
) -> impl Responder {
    println!("Getting swap history");
    let query_str = build_swap_query(&query);
    println!("Generated query: {}", query_str);
    let pool = pool.lock().await;
    let rows = sqlx::query_as::<_, SwapHistory>(&query_str)
        .fetch_all(&*pool)
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

    // Time range filters
    if let Some(from) = query.from {
        where_clauses.push(format!("starttime >= {}", from));
    }
    if let Some(to) = query.to {
        where_clauses.push(format!("endtime <= {}", to));
    }

    // Dynamic filters
    add_condition(
        &mut where_clauses,
        "toassetvolume",
        &query.to_asset_volume_gt,
        ">",
    );
    add_condition(
        &mut where_clauses,
        "toassetvolume",
        &query.to_asset_volume_lt,
        "<",
    );
    add_condition(
        &mut where_clauses,
        "toassetvolume",
        &query.to_asset_volume_eq,
        "=",
    );

    add_condition(
        &mut where_clauses,
        "totalvolumeusd",
        &query.total_volume_usd_gt,
        ">",
    );
    add_condition(
        &mut where_clauses,
        "totalvolumeusd",
        &query.total_volume_usd_lt,
        "<",
    );
    add_condition(
        &mut where_clauses,
        "totalvolumeusd",
        &query.total_volume_usd_eq,
        "=",
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

    if let Some(interval) = &query.interval {
        let interval_seconds = match interval.as_str() {
            "hour" => 3600,
            "day" => 86400,
            "week" => 604800,
            "month" => 2592000,
            "year" => 31536000,
            _ => 3600,
        };

        format!(
            r#"
            WITH grouped_data AS (
                SELECT
                    (starttime / {interval_seconds}) * {interval_seconds} AS bracket_start,
                    id,
                    starttime,
                    endtime,
                    toassetcount,
                    torunecount,
                    totradecount,
                    fromtradecount,
                    synthmintcount,
                    synthredeemcount,
                    totalcount,
                    toassetvolume,
                    torunevolume,
                    totradevolume,
                    fromtradevolume,
                    synthmintvolume,
                    synthredeemvolume,
                    totalvolume,
                    toassetvolumeusd::FLOAT8 AS toassetvolumeusd,
                    torunevolumeusd::FLOAT8 AS torunevolumeusd,
                    totradevolumeusd::FLOAT8 AS totradevolumeusd,
                    fromtradevolumeusd::FLOAT8 AS fromtradevolumeusd,
                    synthmintvolumeusd::FLOAT8 AS synthmintvolumeusd,
                    synthredeemvolumeusd::FLOAT8 AS synthredeemvolumeusd,
                    totalvolumeusd::FLOAT8 AS totalvolumeusd,
                    toassetfees,
                    torunefees,
                    totradefees,
                    fromtradefees,
                    synthmintfees,
                    synthredeemfees,
                    totalfees,
                    toassetaverageslip::FLOAT8 AS toassetaverageslip,
                    toruneaverageslip::FLOAT8 AS toruneaverageslip,
                    totradeaverageslip::FLOAT8 AS totradeaverageslip,
                    fromtradeaverageslip::FLOAT8 AS fromtradeaverageslip,
                    synthmintaverageslip::FLOAT8 AS synthmintaverageslip,
                    synthredeemaverageslip::FLOAT8 AS synthredeemaverageslip,
                    averageslip::FLOAT8 AS averageslip,
                    runepriceusd::FLOAT8 AS runepriceusd,
                    ROW_NUMBER() OVER (
                        PARTITION BY (starttime / {interval_seconds})
                        ORDER BY {sort_by} DESC
                    ) AS rank
                FROM swap_history
                WHERE {where_sql}
            )
            SELECT *
            FROM grouped_data
            WHERE rank = 1
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
    } else {
        format!(
            r#"
            SELECT
                id,
                starttime,
                endtime,
                toassetcount,
                torunecount,
                totradecount,
                fromtradecount,
                synthmintcount,
                synthredeemcount,
                totalcount,
                toassetvolume,
                torunevolume,
                totradevolume,
                fromtradevolume,
                synthmintvolume,
                synthredeemvolume,
                totalvolume,
                toassetvolumeusd::FLOAT8 AS toassetvolumeusd,
                torunevolumeusd::FLOAT8 AS torunevolumeusd,
                totradevolumeusd::FLOAT8 AS totradevolumeusd,
                fromtradevolumeusd::FLOAT8 AS fromtradevolumeusd,
                synthmintvolumeusd::FLOAT8 AS synthmintvolumeusd,
                synthredeemvolumeusd::FLOAT8 AS synthredeemvolumeusd,
                totalvolumeusd::FLOAT8 AS totalvolumeusd,
                toassetfees,
                torunefees,
                totradefees,
                fromtradefees,
                synthmintfees,
                synthredeemfees,
                totalfees,
                toassetaverageslip::FLOAT8 AS toassetaverageslip,
                toruneaverageslip::FLOAT8 AS toruneaverageslip,
                totradeaverageslip::FLOAT8 AS totradeaverageslip,
                fromtradeaverageslip::FLOAT8 AS fromtradeaverageslip,
                synthmintaverageslip::FLOAT8 AS synthmintaverageslip,
                synthredeemaverageslip::FLOAT8 AS synthredeemaverageslip,
                averageslip::FLOAT8 AS averageslip,
                runepriceusd::FLOAT8 AS runepriceusd
            FROM swap_history
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

// Helper function for pagination
fn paginate(page: Option<i32>, limit: Option<i32>) -> (i32, i32) {
    let limit = limit.unwrap_or(10).max(1).min(100);
    let offset = page.unwrap_or(1).saturating_sub(1) * limit;
    (limit, offset)
}

use super::utils::{add_condition, paginate};
use crate::models::swap_history::{SwapHistory, SwapQueryParams};
use actix_web::{web, HttpResponse, Responder};
use sqlx::{Pool, Postgres};
use std::sync::Arc;

pub async fn get_swap_history(
    pool: web::Data<Arc<sqlx::PgPool>>,
    query: web::Query<SwapQueryParams>,
) -> impl Responder {
    println!("Getting swap history");
    let query_str = build_swap_query(&query);
    println!("Generated query: {}", query_str);
    let rows = sqlx::query_as::<_, SwapHistory>(&query_str)
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
    let (pagination_limit, offset) = paginate(query.page, query.limit, query.count);
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

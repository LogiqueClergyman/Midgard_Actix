use crate::models::depth_price_history::DepthPriceHistory;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use std::sync::Arc;

#[derive(Deserialize, Debug)]
pub struct QueryParams {
    pub interval: Option<String>,
    pub from: Option<i64>,
    pub to: Option<i64>,
    pub sort_by: Option<String>,
    pub order: Option<String>,
    pub page: Option<i32>,
    pub limit: Option<i32>,
    pub count: Option<i32>,

    // Dynamic conditions for the columns
    pub assetdepth_gt: Option<i64>,
    pub assetdepth_lt: Option<i64>,
    pub assetdepth_eq: Option<i64>,

    pub assetprice_gt: Option<f64>,
    pub assetprice_lt: Option<f64>,
    pub assetprice_eq: Option<f64>,

    pub liquidityunits_gt: Option<i64>,
    pub liquidityunits_lt: Option<i64>,
    pub liquidityunits_eq: Option<i64>,

    pub luvi_gt: Option<f64>,
    pub luvi_lt: Option<f64>,
    pub luvi_eq: Option<f64>,

    pub memberscount_gt: Option<i32>,
    pub memberscount_lt: Option<i32>,
    pub memberscount_eq: Option<i32>,

    pub runedepth_gt: Option<i64>,
    pub runedepth_lt: Option<i64>,
    pub runedepth_eq: Option<i64>,

    pub synthsupply_gt: Option<i64>,
    pub synthsupply_lt: Option<i64>,
    pub synthsupply_eq: Option<i64>,

    pub synthunits_gt: Option<i64>,
    pub synthunits_lt: Option<i64>,
    pub synthunits_eq: Option<i64>,

    pub units_gt: Option<i64>,
    pub units_lt: Option<i64>,
    pub units_eq: Option<i64>,
}

pub async fn get_depth_price_history(
    pool: web::Data<Arc<tokio::sync::Mutex<Pool<Postgres>>>>,
    query: web::Query<QueryParams>,
) -> impl Responder {
    println!("getting_depth_price_history");
    let query_str = build_query(&query);
    println!("connecting: {}", query_str);
    let pool = pool.lock().await;
    let rows = sqlx::query_as::<_, DepthPriceHistory>(&query_str)
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

    // Add conditions for time range
    if let Some(from) = query.from {
        where_clauses.push(format!("starttime >= {}", from));
    }

    if let Some(to) = query.to {
        where_clauses.push(format!("endtime <= {}", to));
    }

    // Add conditions for greater than, less than, equal to for each field
    add_condition(&mut where_clauses, "assetdepth", &query.assetdepth_gt, ">");
    add_condition(&mut where_clauses, "assetdepth", &query.assetdepth_lt, "<");
    add_condition(&mut where_clauses, "assetdepth", &query.assetdepth_eq, "=");

    add_condition(&mut where_clauses, "assetprice", &query.assetprice_gt, ">");
    add_condition(&mut where_clauses, "assetprice", &query.assetprice_lt, "<");
    add_condition(&mut where_clauses, "assetprice", &query.assetprice_eq, "=");

    add_condition(
        &mut where_clauses,
        "liquidityunits",
        &query.liquidityunits_gt,
        ">",
    );
    add_condition(
        &mut where_clauses,
        "liquidityunits",
        &query.liquidityunits_lt,
        "<",
    );
    add_condition(
        &mut where_clauses,
        "liquidityunits",
        &query.liquidityunits_eq,
        "=",
    );

    add_condition(&mut where_clauses, "luvi", &query.luvi_gt, ">");
    add_condition(&mut where_clauses, "luvi", &query.luvi_lt, "<");
    add_condition(&mut where_clauses, "luvi", &query.luvi_eq, "=");

    add_condition(
        &mut where_clauses,
        "memberscount",
        &query.memberscount_gt,
        ">",
    );
    add_condition(
        &mut where_clauses,
        "memberscount",
        &query.memberscount_lt,
        "<",
    );
    add_condition(
        &mut where_clauses,
        "memberscount",
        &query.memberscount_eq,
        "=",
    );

    add_condition(&mut where_clauses, "runedepth", &query.runedepth_gt, ">");
    add_condition(&mut where_clauses, "runedepth", &query.runedepth_lt, "<");
    add_condition(&mut where_clauses, "runedepth", &query.runedepth_eq, "=");

    add_condition(
        &mut where_clauses,
        "synthsupply",
        &query.synthsupply_gt,
        ">",
    );
    add_condition(
        &mut where_clauses,
        "synthsupply",
        &query.synthsupply_lt,
        "<",
    );
    add_condition(
        &mut where_clauses,
        "synthsupply",
        &query.synthsupply_eq,
        "=",
    );

    add_condition(&mut where_clauses, "synthunits", &query.synthunits_gt, ">");
    add_condition(&mut where_clauses, "synthunits", &query.synthunits_lt, "<");
    add_condition(&mut where_clauses, "synthunits", &query.synthunits_eq, "=");

    add_condition(&mut where_clauses, "units", &query.units_gt, ">");
    add_condition(&mut where_clauses, "units", &query.units_lt, "<");
    add_condition(&mut where_clauses, "units", &query.units_eq, "=");

    let where_sql = if where_clauses.is_empty() {
        "TRUE".to_string()
    } else {
        where_clauses.join(" AND ")
    };
    if let Some(count) = query.count {
        where_clauses.push(format!("count = {}", count));
    }
    println!("{}", where_sql);

    // Sorting and Pagination logic
    let sort_by = query
        .sort_by
        .clone()
        .unwrap_or_else(|| "starttime".to_string());
    let order = query.order.clone().unwrap_or_else(|| "asc".to_string());
    let order_sql = if order == "desc" { "DESC" } else { "ASC" };
    let hard_limit = query.count.unwrap_or(400).min(400);
    let (pagination_limit, offset) = paginate(query.page, query.limit);
    let effective_limit = hard_limit.min(pagination_limit);
    let sort_by = query.sort_by.clone().unwrap_or_else(|| "starttime".to_string());
    if let Some(interval) = &query.interval {
        let interval_seconds = match interval.as_str() {
            "hour" => 3600,
            "day" => 86400,
            "week" => 604800,
            "month" => 2592000,
            "year" => 31536000,
            _ => 3600,
        };

        if interval == "hour" {
            format!(
                r#"
                    SELECT starttime, endtime, assetpriceusd::FLOAT8 AS assetpriceusd, assetdepth, assetprice::FLOAT8 AS assetprice, liquidityunits, luvi::FLOAT8 AS luvi, memberscount, runedepth, synthsupply, synthunits, units
                    FROM depth_price_history
                    WHERE {}
                    ORDER BY {} {}
                    LIMIT {} OFFSET {}
                "#,
                where_sql, sort_by, order_sql, effective_limit, offset
            )
        } else {
            format!(
               r#"
    WITH grouped_data AS (
        SELECT
            (starttime / {interval_seconds}) * {interval_seconds} AS bracket_start,
            starttime, 
            endtime, 
            assetpriceusd::FLOAT8 AS assetpriceusd, 
            assetdepth, 
            assetprice::FLOAT8 AS assetprice, 
            liquidityunits, 
            luvi::FLOAT8 AS luvi, 
            memberscount, 
            runedepth, 
            synthsupply, 
            synthunits, 
            units,
            ROW_NUMBER() OVER (
                PARTITION BY (starttime / {interval_seconds})
                ORDER BY {sort_by} DESC -- Order by dynamic sort column
            ) AS rank
        FROM depth_price_history
        WHERE {where_sql}
    )
    SELECT
        MIN(bracket_start) AS starttime,  -- First time of the bracket
        MAX(starttime) AS endtime,        -- Last time of the bracket
        MIN(starttime) AS first_starttime, -- First starttime in the bracket
        MAX(endtime) AS last_endtime,     -- Last endtime in the bracket
        assetpriceusd::FLOAT8 AS assetpriceusd, 
        assetdepth, 
        assetprice::FLOAT8 AS assetprice, 
        liquidityunits, 
        luvi::FLOAT8 AS luvi, 
        memberscount, 
        runedepth, 
        synthsupply, 
        synthunits, 
        units
    FROM grouped_data
    WHERE rank = 1
    GROUP BY bracket_start, assetdepth, assetprice, assetpriceusd, liquidityunits, luvi, memberscount, runedepth, synthsupply, synthunits, units
    ORDER BY {sort_by} {order_sql}  -- Apply sorting by the selected column here as well
    LIMIT {limit} OFFSET {offset}
"#
,
                interval_seconds = interval_seconds,
                where_sql = where_sql,
                order_sql = order_sql,
                limit = effective_limit,
                offset = offset,
                sort_by = sort_by
            )
        }
    } else {
        format!(
            r#"
                SELECT starttime, endtime, assetpriceusd::FLOAT8 AS assetpriceusd, assetdepth, assetprice::FLOAT8 AS assetprice, liquidityunits, luvi::FLOAT8 AS luvi, memberscount, runedepth, synthsupply, synthunits, units
                FROM depth_price_history
                WHERE {}
                ORDER BY {} {}
                LIMIT {} OFFSET {}
            "#,
            where_sql, sort_by, order_sql, effective_limit, offset
        )
    }
}

fn add_condition<T: std::fmt::Display>(
    where_clauses: &mut Vec<String>,
    col: &str,
    value: &Option<T>,
    operator: &str,
) {
    if let Some(v) = value {
        where_clauses.push(format!("{} {} {}", col, operator, v));
    }
}

fn paginate(page: Option<i32>, limit: Option<i32>) -> (i32, i32) {
    let page = page.unwrap_or(1);
    let limit = limit.unwrap_or(100);
    let offset = (page - 1) * limit;
    (limit, offset)
}

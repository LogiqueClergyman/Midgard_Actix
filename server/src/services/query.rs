// src/services/query_service.rs
use crate::routes::runepool_history::QueryParams;

pub fn build_query(query: &QueryParams) -> String {
    let mut where_clauses = vec![
        "startTime >= $1".to_string(),
        "endTime <= $2".to_string(),
    ];

    if let Some(units_gt) = query.units_gt {
        where_clauses.push(format!("units > {}", units_gt));
    }

    if let Some(count_gt) = query.count_gt {
        where_clauses.push(format!("count > {}", count_gt));
    }

    let where_sql = where_clauses.join(" AND ");
    let sort_by = query.sort_by.clone().unwrap_or_else(|| "startTime".to_string());
    let order = query.order.clone().unwrap_or_else(|| "asc".to_string());
    let order_sql = if order == "desc" { "DESC" } else { "ASC" };

    format!(
        r#"
        SELECT startTime, endTime, units, count
        FROM runepool_history
        WHERE {} 
        ORDER BY {} {}
        LIMIT $3 OFFSET $4
        "#,
        where_sql, sort_by, order_sql
    )
}

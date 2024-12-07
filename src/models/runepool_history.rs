use crate::models::utils::parse_i64;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
#[derive(Serialize, Deserialize, Debug, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct RunepoolHistory {
    #[serde(deserialize_with = "parse_i64")]
    pub start_time: i64,
    #[serde(deserialize_with = "parse_i64")]
    pub end_time: i64,
    #[serde(deserialize_with = "parse_i64")]
    pub units: i64,
    #[serde(deserialize_with = "parse_i64")]
    pub count: i64,
}
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
    pub units_gt: Option<i64>,
    pub units_lt: Option<i64>,
    pub units_eq: Option<i64>,
}

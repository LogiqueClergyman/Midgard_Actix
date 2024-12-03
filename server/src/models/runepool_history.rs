// src/models/runepool_history.rs
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct RunepoolHistory {
    pub starttime: i64,
    pub endtime: i64,
    pub units: i64,
    pub count: i32,
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
    pub units_lt: Option<i64>,
    pub units_eq: Option<i64>,
    pub count_lt: Option<i32>,
    pub count_eq: Option<i32>,
    pub units_gt: Option<i64>,
    pub count_gt: Option<i32>,
}

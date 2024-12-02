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
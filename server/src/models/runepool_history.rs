// src/models/runepool_history.rs
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct RunepoolHistory {
    pub starttime: i64,  // Adjusted to match database column name
    pub endtime: i64,    // Adjusted to match database column name
    pub units: i64,
    pub count: i32,
}
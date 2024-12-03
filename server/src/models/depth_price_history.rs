// src/models/depth_price_history.rs
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow; // This is to handle DECIMAL(18, 8) type in the database

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct DepthPriceHistory {
    pub assetdepth: i64,           // The asset depth
    pub assetprice: f64,       // The asset price
    pub assetpriceusd: f64,    // The asset price in USD
    pub liquidityunits: i64,       // The liquidity units
    pub luvi: f64,             // The LUVI value
    pub memberscount: i32,         // The number of members
    pub runedepth: i64,            // The rune depth
    pub synthsupply: i64,          // The synth supply
    pub synthunits: i64,           // The synth units
    pub units: i64,                // The units
    pub starttime: i64,            // The start time as UNIX timestamp (seconds)
    pub endtime: i64,              // The end time as UNIX timestamp (seconds)
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

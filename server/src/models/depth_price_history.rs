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


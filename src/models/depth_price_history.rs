use crate::models::utils::{parse_f64, parse_i64, serialize_bigdecimal};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
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
#[derive(Serialize, Deserialize, Debug, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct DepthPriceHistory {
    #[serde(deserialize_with = "parse_i64")]
    pub asset_depth: i64,

    #[serde(
        deserialize_with = "parse_f64",
        serialize_with = "serialize_bigdecimal"
    )]
    pub asset_price: BigDecimal,

    #[serde(
        rename = "assetPriceUSD",
        deserialize_with = "parse_f64",
        serialize_with = "serialize_bigdecimal"
    )]
    pub asset_price_usd: BigDecimal,

    #[serde(deserialize_with = "parse_i64")]
    pub liquidity_units: i64,

    #[serde(
        deserialize_with = "parse_f64",
        serialize_with = "serialize_bigdecimal"
    )]
    pub luvi: BigDecimal,

    #[serde(deserialize_with = "parse_i64")]
    pub members_count: i64,

    #[serde(deserialize_with = "parse_i64")]
    pub rune_depth: i64,

    #[serde(deserialize_with = "parse_i64")]
    pub synth_supply: i64,

    #[serde(deserialize_with = "parse_i64")]
    pub synth_units: i64,

    #[serde(deserialize_with = "parse_i64")]
    pub units: i64,

    #[serde(deserialize_with = "parse_i64")]
    pub start_time: i64,

    #[serde(deserialize_with = "parse_i64")]
    pub end_time: i64,
}

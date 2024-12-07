use crate::models::utils::{parse_f64, parse_i64, serialize_bigdecimal};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::prelude::{FromRow, Type};
#[derive(Deserialize, Serialize, FromRow, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EarningHistory {
    #[serde(
        deserialize_with = "parse_f64",
        serialize_with = "serialize_bigdecimal"
    )]
    pub avg_node_count: BigDecimal,
    #[serde(deserialize_with = "parse_i64")]
    pub block_rewards: i64,
    #[serde(deserialize_with = "parse_i64")]
    pub bonding_earnings: i64,
    #[serde(deserialize_with = "parse_i64")]
    pub earnings: i64,
    #[serde(deserialize_with = "parse_i64")]
    pub end_time: i64,
    #[serde(deserialize_with = "parse_i64")]
    pub liquidity_earnings: i64,
    #[serde(deserialize_with = "parse_i64")]
    pub liquidity_fees: i64,
    #[serde(
        deserialize_with = "parse_f64",
        serialize_with = "serialize_bigdecimal"
    )]
    #[serde(rename = "runePriceUSD")]
    pub rune_price_usd: BigDecimal,
    #[serde(deserialize_with = "parse_i64")]
    pub start_time: i64,
    pub pool: Option<Vec<Value>>,
}

#[derive(Deserialize, Serialize, Type, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct EarningHistoryNested {
    pub pool: String,
    #[serde(deserialize_with = "parse_i64")]
    pub asset_liquidity_fees: i64,
    #[serde(deserialize_with = "parse_i64")]
    pub rune_liquidity_fees: i64,
    #[serde(deserialize_with = "parse_i64")]
    pub saver_earning: i64,
    #[serde(deserialize_with = "parse_i64")]
    pub total_liquidity_fees_rune: i64,
    #[serde(deserialize_with = "parse_i64")]
    pub earnings: i64,
    #[serde(deserialize_with = "parse_i64")]
    pub rewards: i64,
}

#[derive(Deserialize, Debug)]
pub struct EarningHistoryQueryParams {
    pub interval: Option<String>,
    pub from: Option<i64>,
    pub to: Option<i64>,
    pub sort_by: Option<String>,
    pub order: Option<String>,
    pub page: Option<i32>,
    pub limit: Option<i32>,
    pub count: Option<i32>,

    // Dynamic filters for earning_history fields
    pub avg_node_count_gt: Option<f64>,
    pub avg_node_count_lt: Option<f64>,
    pub block_rewards_gt: Option<i64>,
    pub block_rewards_lt: Option<i64>,

    pub earnings_gt: Option<i64>,
    pub earnings_lt: Option<i64>,
    pub liquidity_earnings_gt: Option<i64>,
    pub liquidity_earnings_lt: Option<i64>,

    pub rune_price_usd_gt: Option<f64>,
    pub rune_price_usd_lt: Option<f64>,
    pub rune_price_usd_eq: Option<f64>,
}

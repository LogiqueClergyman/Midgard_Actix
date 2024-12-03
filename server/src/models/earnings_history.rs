use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::prelude::{FromRow, Type};
#[derive(Serialize, Deserialize, FromRow, Debug, Type)]
pub struct EarningHistoryResponse {
    pub avgnodecount: f64,
    pub blockrewards: i64,
    pub bondingearnings: i64,
    pub earnings: i64,
    pub endtime: i64,
    pub liquidityearnings: i64,
    pub liquidityfees: i64,
    pub runepriceusd: f64,
    pub starttime: i64,
    pub pools: Vec<Value>,
}

#[derive(Serialize, Deserialize, FromRow, Debug, Type)]
#[sqlx(type_name = "earning_history_nested")]
pub struct EarningHistoryNestedResponse {
    pub pool: String,
    pub asset_liquidity_fees: i64,
    pub earnings: i64,
    pub rewards: i64,
    pub rune_liquidity_fees: i64,
    pub saver_earning: i64,
    pub total_liquidity_fees_rune: i64,
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

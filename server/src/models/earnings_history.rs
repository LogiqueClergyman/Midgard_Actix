use serde::{Deserialize, Serialize};
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
    pub pools: Option<Vec<EarningHistoryNestedResponse>>,
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

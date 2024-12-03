use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize)]
pub struct EarningHistoryNestedData {
    pub pool: String,
    #[serde(deserialize_with = "deserialize_i64")]
    pub assetLiquidityFees: i64,
    #[serde(deserialize_with = "deserialize_i64")]
    pub earnings: i64,
    #[serde(deserialize_with = "deserialize_i64")]
    pub rewards: i64,
    #[serde(deserialize_with = "deserialize_i64")]
    pub runeLiquidityFees: i64,
    #[serde(deserialize_with = "deserialize_i64")]
    pub saverEarning: i64,
    #[serde(deserialize_with = "deserialize_i64")]
    pub totalLiquidityFeesRune: i64,
}
fn deserialize_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Value = Deserialize::deserialize(deserializer)?;
    match value {
        Value::String(s) => s.parse::<i64>().map_err(serde::de::Error::custom), // Try to parse string as i64
        Value::Number(n) => n.as_i64().ok_or_else(|| serde::de::Error::custom("invalid i64 number")), // If it's already a number
        _ => Err(serde::de::Error::custom("expected string or number")),
    }
}
#[derive(Deserialize, Serialize)]
pub struct EarningHistoryData {
    pub avgNodeCount: f64,
    pub blockRewards: i64,
    pub bondingEarnings: i64,
    pub earnings: i64,
    pub endTime: i64,
    pub liquidityEarnings: i64,
    pub liquidityFees: i64,
    pub runePriceUsd: f64,
    pub startTime: i64,
    pub pool: Vec<i32>,
}

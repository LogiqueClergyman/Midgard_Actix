use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetPriceHistory {
    pub id: Option<i32>,
    pub assetDepth: i64,
    pub assetPrice: f64,
    pub assetPriceUSD: f64,
    pub liquidityUnits: i64,
    pub luvi: f64,
    pub membersCount: i32,
    pub runeDepth: i64,
    pub synthSupply: i64,
    pub synthUnits: i64,
    pub units: i64,
    pub startTime: i64,
    pub endTime: i64,
}

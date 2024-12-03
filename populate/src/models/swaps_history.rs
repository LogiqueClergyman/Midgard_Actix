use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SwapsHistory {
    pub id: Option<i32>,
    pub startTime: i64,              // UNIX timestamp as i64
    pub endTime: i64,                // UNIX timestamp as i64
    pub toAssetCount: i64,           // Swap count (rune to asset)
    pub toRuneCount: i64,            // Swap count (asset to rune)
    pub toTradeCount: i64,           // Swap count (rune to trade asset)
    pub fromTradeCount: i64,         // Swap count (trade asset to rune)
    pub synthMintCount: i64,         // Swap count (rune to synthetic asset)
    pub synthRedeemCount: i64,       // Swap count (synthetic asset to rune)
    pub totalCount: i64,             // Total swap count
    pub toAssetVolume: i64,          // Volume of swaps from rune to asset in rune
    pub toRuneVolume: i64,           // Volume of swaps from asset to rune in rune
    pub toTradeVolume: i64,          // Volume of swaps from rune to trade asset in rune
    pub fromTradeVolume: i64,        // Volume of swaps from trade asset to rune in rune
    pub synthMintVolume: i64,        // Volume of swaps from rune to synthetic asset in rune
    pub synthRedeemVolume: i64,      // Volume of swaps from synthetic asset to rune in rune
    pub totalVolume: i64,            // Total volume in rune (sum of all volumes)
    pub toAssetVolumeUSD: f64,       // Volume in USD for rune to asset swaps
    pub toRuneVolumeUSD: f64,        // Volume in USD for asset to rune swaps
    pub toTradeVolumeUSD: f64,       // Volume in USD for rune to trade asset swaps
    pub fromTradeVolumeUSD: f64,     // Volume in USD for trade asset to rune swaps
    pub synthMintVolumeUSD: f64,     // Volume in USD for rune to synthetic asset swaps
    pub synthRedeemVolumeUSD: f64,   // Volume in USD for synthetic asset to rune swaps
    pub totalVolumeUSD: f64,         // Total volume in USD
    pub toAssetFees: i64,            // Fees collected from rune to asset swaps (in rune)
    pub toRuneFees: i64,             // Fees collected from asset to rune swaps (in rune)
    pub toTradeFees: i64,            // Fees collected from rune to trade asset swaps (in rune)
    pub fromTradeFees: i64,          // Fees collected from trade asset to rune swaps (in rune)
    pub synthMintFees: i64,          // Fees collected from rune to synthetic asset swaps (in rune)
    pub synthRedeemFees: i64,        // Fees collected from synthetic asset to rune swaps (in rune)
    pub totalFees: i64,              // Total fees collected (sum of all fees)
    pub toAssetAverageSlip: f64,     // Average slip (basis points) for rune to asset swaps
    pub toRuneAverageSlip: f64,      // Average slip (basis points) for asset to rune swaps
    pub toTradeAverageSlip: f64,     // Average slip (basis points) for rune to trade asset swaps
    pub fromTradeAverageSlip: f64,   // Average slip (basis points) for trade asset to rune swaps
    pub synthMintAverageSlip: f64, // Average slip (basis points) for rune to synthetic asset swaps
    pub synthRedeemAverageSlip: f64, // Average slip (basis points) for synthetic asset to rune swaps
    pub averageSlip: f64,            // Weighted average slip (basis points) for all swaps
    pub runePriceUSD: f64,           // Price of Rune in USD
}

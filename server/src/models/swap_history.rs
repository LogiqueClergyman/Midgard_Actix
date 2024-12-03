use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct SwapHistory {
    pub starttime: i64,                   // Start time as UNIX timestamp (seconds)
    pub endtime: i64,                     // End time as UNIX timestamp (seconds)
    pub toassetcount: i64,               // Count of swaps from rune to asset
    pub torunecount: i64,                // Count of swaps from asset to rune
    pub totradecount: i64,               // Count of swaps from rune to trade asset
    pub fromtradecount: i64,             // Count of swaps from trade asset to rune
    pub synthmintcount: i64,             // Count of swaps from rune to synthetic asset
    pub synthredeemcount: i64,           // Count of swaps from synthetic asset to rune
    pub totalcount: i64,                 // Total swap count
    pub toassetvolume: i64,              // Volume of swaps from rune to asset in rune
    pub torunevolume: i64,               // Volume of swaps from asset to rune in rune
    pub totradevolume: i64,              // Volume of swaps from rune to trade asset in rune
    pub fromtradevolume: i64,            // Volume of swaps from trade asset to rune in rune
    pub synthmintvolume: i64,            // Volume of swaps from rune to synthetic asset in rune
    pub synthredeemvolume: i64,          // Volume of swaps from synthetic asset to rune in rune
    pub totalvolume: i64,                // Total volume in rune
    pub toassetvolumeusd: f64,           // Volume in USD for rune to asset swaps
    pub torunevolumeusd: f64,            // Volume in USD for asset to rune swaps
    pub totradevolumeusd: f64,           // Volume in USD for rune to trade asset swaps
    pub fromtradevolumeusd: f64,         // Volume in USD for trade asset to rune swaps
    pub synthmintvolumeusd: f64,         // Volume in USD for rune to synthetic asset swaps
    pub synthredeemvolumeusd: f64,       // Volume in USD for synthetic asset to rune swaps
    pub totalvolumeusd: f64,             // Total volume in USD
    pub toassetfees: i64,                // Fees collected from rune to asset swaps (in rune)
    pub torunefees: i64,                 // Fees collected from asset to rune swaps (in rune)
    pub totradefees: i64,                // Fees collected from rune to trade asset swaps (in rune)
    pub fromtradefees: i64,              // Fees collected from trade asset to rune swaps (in rune)
    pub synthmintfees: i64,              // Fees collected from rune to synthetic asset swaps (in rune)
    pub synthredeemfees: i64,            // Fees collected from synthetic asset to rune swaps (in rune)
    pub totalfees: i64,                  // Total fees collected (sum of all fees)
    pub toassetaverageslip: f64,         // Average slip (basis points) for rune to asset swaps
    pub toruneaverageslip: f64,          // Average slip (basis points) for asset to rune swaps
    pub totradeaverageslip: f64,         // Average slip (basis points) for rune to trade asset swaps
    pub fromtradeaverageslip: f64,       // Average slip (basis points) for trade asset to rune swaps
    pub synthmintaverageslip: f64,       // Average slip (basis points) for rune to synthetic asset swaps
    pub synthredeemaverageslip: f64,     // Average slip (basis points) for synthetic asset to rune swaps
    pub averageslip: f64,                // Weighted average slip (basis points) for all swaps
    pub runepriceusd: f64,               // Price of Rune in USD
}

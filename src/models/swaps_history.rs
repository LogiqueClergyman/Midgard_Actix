use crate::models::utils::{parse_f64, parse_i64, serialize_bigdecimal};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
#[derive(Serialize, Deserialize, Debug, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct SwapsHistory {
    #[serde(deserialize_with = "parse_i64")]
    pub start_time: i64,
    #[serde(deserialize_with = "parse_i64")]
    pub end_time: i64,
    #[serde(deserialize_with = "parse_i64")]
    pub to_asset_count: i64,
    #[serde(deserialize_with = "parse_i64")]
    pub to_rune_count: i64,
    #[serde(deserialize_with = "parse_i64")]
    pub to_trade_count: i64,
    #[serde(deserialize_with = "parse_i64")]
    pub from_trade_count: i64,
    #[serde(deserialize_with = "parse_i64")]
    pub synth_mint_count: i64,
    #[serde(deserialize_with = "parse_i64")]
    pub synth_redeem_count: i64,
    #[serde(deserialize_with = "parse_i64")]
    pub total_count: i64,
    #[serde(deserialize_with = "parse_i64")]
    pub to_asset_volume: i64,
    #[serde(deserialize_with = "parse_i64")]
    pub to_rune_volume: i64,
    #[serde(deserialize_with = "parse_i64")]
    pub to_trade_volume: i64,
    #[serde(deserialize_with = "parse_i64")]
    pub from_trade_volume: i64,
    #[serde(deserialize_with = "parse_i64")]
    pub synth_mint_volume: i64,
    #[serde(deserialize_with = "parse_i64")]
    pub synth_redeem_volume: i64,
    #[serde(deserialize_with = "parse_i64")]
    pub total_volume: i64,
    #[serde(rename = "toAssetVolumeUSD")]
    #[serde(
        deserialize_with = "parse_f64",
        serialize_with = "serialize_bigdecimal"
    )]
    pub to_asset_volume_usd: BigDecimal,
    #[serde(rename = "toRuneVolumeUSD")]
    #[serde(
        deserialize_with = "parse_f64",
        serialize_with = "serialize_bigdecimal"
    )]
    pub to_rune_volume_usd: BigDecimal,
    #[serde(rename = "toTradeVolumeUSD")]
    #[serde(
        deserialize_with = "parse_f64",
        serialize_with = "serialize_bigdecimal"
    )]
    pub to_trade_volume_usd: BigDecimal,
    #[serde(rename = "fromTradeVolumeUSD")]
    #[serde(
        deserialize_with = "parse_f64",
        serialize_with = "serialize_bigdecimal"
    )]
    pub from_trade_volume_usd: BigDecimal,
    #[serde(rename = "synthMintVolumeUSD")]
    #[serde(
        deserialize_with = "parse_f64",
        serialize_with = "serialize_bigdecimal"
    )]
    pub synth_mint_volume_usd: BigDecimal,
    #[serde(rename = "synthRedeemVolumeUSD")]
    #[serde(
        deserialize_with = "parse_f64",
        serialize_with = "serialize_bigdecimal"
    )]
    pub synth_redeem_volume_usd: BigDecimal,
    #[serde(rename = "totalVolumeUSD")]
    #[serde(
        deserialize_with = "parse_f64",
        serialize_with = "serialize_bigdecimal"
    )]
    pub total_volume_usd: BigDecimal,
    #[serde(deserialize_with = "parse_i64")]
    pub to_asset_fees: i64,
    #[serde(deserialize_with = "parse_i64")]
    pub to_rune_fees: i64,
    #[serde(deserialize_with = "parse_i64")]
    pub to_trade_fees: i64,
    #[serde(deserialize_with = "parse_i64")]
    pub from_trade_fees: i64,
    #[serde(deserialize_with = "parse_i64")]
    pub synth_mint_fees: i64,
    #[serde(deserialize_with = "parse_i64")]
    pub synth_redeem_fees: i64,
    #[serde(deserialize_with = "parse_i64")]
    pub total_fees: i64,
    #[serde(
        deserialize_with = "parse_f64",
        serialize_with = "serialize_bigdecimal"
    )]
    pub to_asset_average_slip: BigDecimal,
    #[serde(
        deserialize_with = "parse_f64",
        serialize_with = "serialize_bigdecimal"
    )]
    pub to_rune_average_slip: BigDecimal,
    #[serde(
        deserialize_with = "parse_f64",
        serialize_with = "serialize_bigdecimal"
    )]
    pub to_trade_average_slip: BigDecimal,
    #[serde(
        deserialize_with = "parse_f64",
        serialize_with = "serialize_bigdecimal"
    )]
    pub from_trade_average_slip: BigDecimal,
    #[serde(
        deserialize_with = "parse_f64",
        serialize_with = "serialize_bigdecimal"
    )]
    pub synth_mint_average_slip: BigDecimal,
    #[serde(
        deserialize_with = "parse_f64",
        serialize_with = "serialize_bigdecimal"
    )]
    pub synth_redeem_average_slip: BigDecimal,
    #[serde(
        deserialize_with = "parse_f64",
        serialize_with = "serialize_bigdecimal"
    )]
    pub average_slip: BigDecimal,
    #[serde(
        deserialize_with = "parse_f64",
        serialize_with = "serialize_bigdecimal"
    )]
    #[serde(rename = "runePriceUSD")]
    pub rune_price_usd: BigDecimal,
}

#[derive(Deserialize, Debug)]
pub struct SwapQueryParams {
    pub interval: Option<String>,
    pub from: Option<i64>,
    pub to: Option<i64>,
    pub sort_by: Option<String>,
    pub order: Option<String>,
    pub page: Option<i32>,
    pub limit: Option<i32>,
    pub count: Option<i32>,

    pub to_asset_volume_gt: Option<i64>,
    pub to_asset_volume_lt: Option<i64>,
    pub to_asset_volume_eq: Option<i64>,

    pub total_volume_usd_gt: Option<f64>,
    pub total_volume_usd_lt: Option<f64>,
    pub total_volume_usd_eq: Option<f64>,

    pub rune_price_usd_gt: Option<f64>,
    pub rune_price_usd_lt: Option<f64>,
    pub rune_price_usd_eq: Option<f64>,
}

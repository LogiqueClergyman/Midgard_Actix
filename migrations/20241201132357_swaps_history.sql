-- Create table for storing swap history data
CREATE TABLE swap_history (
    id SERIAL PRIMARY KEY,                           -- Auto-incrementing unique ID for each record
    startTime BIGINT NOT NULL,                       -- The start time as UNIX timestamp (seconds)
    endTime BIGINT NOT NULL,                         -- The end time as UNIX timestamp (seconds)
    toAssetCount BIGINT NOT NULL,                    -- Count of swaps from rune to asset
    toRuneCount BIGINT NOT NULL,                     -- Count of swaps from asset to rune
    toTradeCount BIGINT NOT NULL,                    -- Count of swaps from rune to trade asset
    fromTradeCount BIGINT NOT NULL,                  -- Count of swaps from trade asset to rune
    synthMintCount BIGINT NOT NULL,                  -- Count of swaps from rune to synthetic asset
    synthRedeemCount BIGINT NOT NULL,                -- Count of swaps from synthetic asset to rune
    totalCount BIGINT NOT NULL,                      -- Total swap count
    toAssetVolume BIGINT NOT NULL,                   -- Volume of swaps from rune to asset in rune
    toRuneVolume BIGINT NOT NULL,                    -- Volume of swaps from asset to rune in rune
    toTradeVolume BIGINT NOT NULL,                   -- Volume of swaps from rune to trade asset in rune
    fromTradeVolume BIGINT NOT NULL,                 -- Volume of swaps from trade asset to rune in rune
    synthMintVolume BIGINT NOT NULL,                 -- Volume of swaps from rune to synthetic asset in rune
    synthRedeemVolume BIGINT NOT NULL,               -- Volume of swaps from synthetic asset to rune in rune
    totalVolume BIGINT NOT NULL,                     -- Total volume in rune (sum of all volumes)
    toAssetVolumeUSD DECIMAL(18, 8) NOT NULL,        -- Volume in USD for rune to asset swaps
    toRuneVolumeUSD DECIMAL(18, 8) NOT NULL,         -- Volume in USD for asset to rune swaps
    toTradeVolumeUSD DECIMAL(18, 8) NOT NULL,        -- Volume in USD for rune to trade asset swaps
    fromTradeVolumeUSD DECIMAL(18, 8) NOT NULL,      -- Volume in USD for trade asset to rune swaps
    synthMintVolumeUSD DECIMAL(18, 8) NOT NULL,      -- Volume in USD for rune to synthetic asset swaps
    synthRedeemVolumeUSD DECIMAL(18, 8) NOT NULL,    -- Volume in USD for synthetic asset to rune swaps
    totalVolumeUSD DECIMAL(18, 8) NOT NULL,          -- Total volume in USD
    toAssetFees BIGINT NOT NULL,                     -- Fees collected from rune to asset swaps (in rune)
    toRuneFees BIGINT NOT NULL,                      -- Fees collected from asset to rune swaps (in rune)
    toTradeFees BIGINT NOT NULL,                     -- Fees collected from rune to trade asset swaps (in rune)
    fromTradeFees BIGINT NOT NULL,                   -- Fees collected from trade asset to rune swaps (in rune)
    synthMintFees BIGINT NOT NULL,                   -- Fees collected from rune to synthetic asset swaps (in rune)
    synthRedeemFees BIGINT NOT NULL,                 -- Fees collected from synthetic asset to rune swaps (in rune)
    totalFees BIGINT NOT NULL,                       -- Total fees collected (sum of all fees)
    toAssetAverageSlip DECIMAL(18, 8) NOT NULL,      -- Average slip (basis points) for rune to asset swaps
    toRuneAverageSlip DECIMAL(18, 8) NOT NULL,       -- Average slip (basis points) for asset to rune swaps
    toTradeAverageSlip DECIMAL(18, 8) NOT NULL,      -- Average slip (basis points) for rune to trade asset swaps
    fromTradeAverageSlip DECIMAL(18, 8) NOT NULL,    -- Average slip (basis points) for trade asset to rune swaps
    synthMintAverageSlip DECIMAL(18, 8) NOT NULL,    -- Average slip (basis points) for rune to synthetic asset swaps
    synthRedeemAverageSlip DECIMAL(18, 8) NOT NULL,  -- Average slip (basis points) for synthetic asset to rune swaps
    averageSlip DECIMAL(18, 8) NOT NULL,             -- Weighted average slip (basis points) for all swaps
    runePriceUSD DECIMAL(18, 8) NOT NULL,            -- Price of Rune in USD
    UNIQUE (startTime, endTime)                      -- Ensures that each record is unique based on time interval
);

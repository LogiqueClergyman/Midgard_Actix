-- Create table for storing swap history data
CREATE TABLE swap_history (
    id SERIAL PRIMARY KEY,                         -- Unique identifier for each record
    start_time BIGINT NOT NULL,                       -- The start time as UNIX timestamp (seconds)
    end_time BIGINT NOT NULL,                         -- The end time as UNIX timestamp (seconds)
    to_asset_count BIGINT NOT NULL,                   -- Count of swaps from rune to asset
    to_rune_count BIGINT NOT NULL,                    -- Count of swaps from asset to rune
    to_trade_count BIGINT NOT NULL,                   -- Count of swaps from rune to trade asset
    from_trade_count BIGINT NOT NULL,                 -- Count of swaps from trade asset to rune
    synth_mint_count BIGINT NOT NULL,                 -- Count of swaps from rune to synthetic asset
    synth_redeem_count BIGINT NOT NULL,               -- Count of swaps from synthetic asset to rune
    total_count BIGINT NOT NULL,                      -- Total swap count
    to_asset_volume BIGINT NOT NULL,                  -- Volume of swaps from rune to asset in rune
    to_rune_volume BIGINT NOT NULL,                   -- Volume of swaps from asset to rune in rune
    to_trade_volume BIGINT NOT NULL,                  -- Volume of swaps from rune to trade asset in rune
    from_trade_volume BIGINT NOT NULL,                -- Volume of swaps from trade asset to rune in rune
    synth_mint_volume BIGINT NOT NULL,                -- Volume of swaps from rune to synthetic asset in rune
    synth_redeem_volume BIGINT NOT NULL,              -- Volume of swaps from synthetic asset to rune in rune
    total_volume BIGINT NOT NULL,                     -- Total volume in rune (sum of all volumes)
    to_asset_volume_usd DECIMAL(18, 8) NOT NULL,      -- Volume in USD for rune to asset swaps
    to_rune_volume_usd DECIMAL(18, 8) NOT NULL,       -- Volume in USD for asset to rune swaps
    to_trade_volume_usd DECIMAL(18, 8) NOT NULL,      -- Volume in USD for rune to trade asset swaps
    from_trade_volume_usd DECIMAL(18, 8) NOT NULL,    -- Volume in USD for trade asset to rune swaps
    synth_mint_volume_usd DECIMAL(18, 8) NOT NULL,    -- Volume in USD for rune to synthetic asset swaps
    synth_redeem_volume_usd DECIMAL(18, 8) NOT NULL,  -- Volume in USD for synthetic asset to rune swaps
    total_volume_usd DECIMAL(18, 8) NOT NULL,         -- Total volume in USD
    to_asset_fees BIGINT NOT NULL,                    -- Fees collected from rune to asset swaps (in rune)
    to_rune_fees BIGINT NOT NULL,                     -- Fees collected from asset to rune swaps (in rune)
    to_trade_fees BIGINT NOT NULL,                    -- Fees collected from rune to trade asset swaps (in rune)
    from_trade_fees BIGINT NOT NULL,                  -- Fees collected from trade asset to rune swaps (in rune)
    synth_mint_fees BIGINT NOT NULL,                  -- Fees collected from rune to synthetic asset swaps (in rune)
    synth_redeem_fees BIGINT NOT NULL,                -- Fees collected from synthetic asset to rune swaps (in rune)
    total_fees BIGINT NOT NULL,                       -- Total fees collected (sum of all fees)
    to_asset_average_slip DECIMAL(18, 8) NOT NULL,    -- Average slip (basis points) for rune to asset swaps
    to_rune_average_slip DECIMAL(18, 8) NOT NULL,     -- Average slip (basis points) for asset to rune swaps
    to_trade_average_slip DECIMAL(18, 8) NOT NULL,    -- Average slip (basis points) for rune to trade asset swaps
    from_trade_average_slip DECIMAL(18, 8) NOT NULL,  -- Average slip (basis points) for trade asset to rune swaps
    synth_mint_average_slip DECIMAL(18, 8) NOT NULL,  -- Average slip (basis points) for rune to synthetic asset swaps
    synth_redeem_average_slip DECIMAL(18, 8) NOT NULL,-- Average slip (basis points) for synthetic asset to rune swaps
    average_slip DECIMAL(18, 8) NOT NULL,             -- Weighted average slip (basis points) for all swaps
    rune_price_usd DECIMAL(18, 8) NOT NULL,           -- Price of Rune in USD
    UNIQUE (start_time, end_time)                     -- Ensures that each record is unique based on time interval
);

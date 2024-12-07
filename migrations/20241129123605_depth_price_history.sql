-- Add migration script here
CREATE TABLE depth_price_history (
    id SERIAL PRIMARY KEY,                  -- Auto-incrementing unique ID for each record
    asset_depth BIGINT,                     -- The asset depth, as returned by the API
    asset_price DECIMAL(18, 8),             -- The asset price, as returned by the API
    asset_price_usd DECIMAL(18, 8),         -- The asset price in USD, as returned by the API
    liquidity_units BIGINT,                 -- The liquidity units, as returned by the API
    luvi DECIMAL(18, 8),                    -- The LUVI value, as returned by the API
    members_count BIGINT,                   -- The number of members, as returned by the API
    rune_depth BIGINT,                      -- The rune depth, as returned by the API
    synth_supply BIGINT,                    -- The synth supply, as returned by the API
    synth_units BIGINT,                     -- The synth units, as returned by the API
    units BIGINT,                           -- The units, as returned by the API
    start_time BIGINT,                      -- The start time as UNIX timestamp (seconds)
    end_time BIGINT,                        -- The end time as UNIX timestamp (seconds)
    UNIQUE (start_time, end_time)           -- Ensures that each record is unique based on time interval
);
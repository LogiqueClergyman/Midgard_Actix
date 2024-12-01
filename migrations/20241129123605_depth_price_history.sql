-- Add migration script here
CREATE TABLE Depth_Price_History (
    id SERIAL PRIMARY KEY,                  -- Auto-incrementing unique ID for each record
    assetDepth BIGINT,                      -- The asset depth, as returned by the API
    assetPrice DECIMAL(18, 8),              -- The asset price, as returned by the API
    assetPriceUSD DECIMAL(18, 8),           -- The asset price in USD, as returned by the API
    liquidityUnits BIGINT,                  -- The liquidity units, as returned by the API
    luvi DECIMAL(18, 8),                    -- The LUVI value, as returned by the API
    membersCount INT,                       -- The number of members, as returned by the API
    runeDepth BIGINT,                       -- The rune depth, as returned by the API
    synthSupply BIGINT,                     -- The synth supply, as returned by the API
    synthUnits BIGINT,                      -- The synth units, as returned by the API
    units BIGINT,                           -- The units, as returned by the API
    startTime BIGINT,                       -- The start time as UNIX timestamp (seconds)
    endTime BIGINT,                         -- The end time as UNIX timestamp (seconds)
    UNIQUE (startTime, endTime)             -- Ensures that each record is unique based on time interval
);
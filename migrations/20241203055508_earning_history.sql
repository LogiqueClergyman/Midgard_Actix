CREATE TABLE earning_history (
    id SERIAL PRIMARY KEY,
    avgNodeCount DECIMAL(18, 8),
    blockRewards BIGINT,
    bondingEarnings BIGINT,
    earnings BIGINT,
    endTime BIGINT,
    liquidityEarnings BIGINT,
    liquidityFees BIGINT,
    runePriceUsd DOUBLE PRECISION,
    startTime BIGINT,
    pools INTEGER[],
    UNIQUE (startTime, endTime)
);

CREATE TABLE earning_history_nested (
    id SERIAL PRIMARY KEY,
    pool TEXT,
    assetLiquidityFees BIGINT,
    earnings BIGINT,
    rewards BIGINT,
    runeLiquidityFees BIGINT,
    saverEarning BIGINT,
    totalLiquidityFeesRune BIGINT
);
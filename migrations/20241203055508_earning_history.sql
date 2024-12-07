CREATE TABLE earning_history (
    id SERIAL PRIMARY KEY,
    avg_node_count DECIMAL(18, 8),
    block_rewards BIGINT,
    bonding_earnings BIGINT,
    earnings BIGINT,
    end_time BIGINT,
    liquidity_earnings BIGINT,
    liquidity_fees BIGINT,
    rune_price_usd DECIMAL(18, 8),
    start_time BIGINT,
    UNIQUE (start_time, end_time)
);

CREATE TABLE earning_history_nested (
    id SERIAL PRIMARY KEY,
    earning_history_id INT NOT NULL,
    pool VARCHAR(255),  
    asset_liquidity_fees BIGINT,
    earnings BIGINT,
    rewards BIGINT,
    rune_liquidity_fees BIGINT,
    saver_earning BIGINT,
    total_liquidity_fees_rune BIGINT,
    FOREIGN KEY (earning_history_id) REFERENCES earning_history(id) ON DELETE CASCADE
);
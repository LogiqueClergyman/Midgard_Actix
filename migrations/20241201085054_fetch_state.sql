-- Add migration script here
CREATE TABLE fetch_state (
    id SERIAL PRIMARY KEY,
    table_name VARCHAR(255) UNIQUE NOT NULL,
    last_successful_entry BIGINT NOT NULL
);

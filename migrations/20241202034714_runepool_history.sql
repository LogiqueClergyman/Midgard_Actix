-- Add migration script here
CREATE TABLE runepool_history (
    id SERIAL PRIMARY KEY,                       -- Auto-incrementing unique ID for each record
    start_time BIGINT,                            -- The start time as UNIX timestamp (seconds)
    end_time BIGINT,                              -- The end time as UNIX timestamp (seconds)
    units BIGINT,                                -- The units value from the API
    count BIGINT,                                   -- The count value from the API
    UNIQUE (start_time, end_time)                  -- Ensures that each record is unique based on the time interval
);

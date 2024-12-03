-- Add migration script here
CREATE TABLE runepool_history (
    id SERIAL PRIMARY KEY,                       -- Auto-incrementing unique ID for each record
    startTime BIGINT,                            -- The start time as UNIX timestamp (seconds)
    endTime BIGINT,                              -- The end time as UNIX timestamp (seconds)
    units BIGINT,                                -- The units value from the API
    count INT,                                   -- The count value from the API
    UNIQUE (startTime, endTime)                  -- Ensures that each record is unique based on the time interval
);

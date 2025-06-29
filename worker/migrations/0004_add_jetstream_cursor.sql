-- Migration number: 0004 	 2025-06-26T00:00:00.000Z

CREATE TABLE IF NOT EXISTS jetstream_cursor (
    last_seen_timestamp INTEGER NOT NULL
);
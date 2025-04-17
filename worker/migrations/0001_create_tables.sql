-- Migration number: 0001 	 2025-04-17T02:35:29.320Z
CREATE TABLE IF NOT EXISTS status (
uri TEXT PRIMARY KEY,
authorDid TEXT NOT NULL,
status TEXT NOT NULL,
source TEXT NOT NULL, -- todo: two separate flags, remove complex update logic
createdAt INTEGER  NOT NULL,
indexedAt INTEGER  NOT NULL
);
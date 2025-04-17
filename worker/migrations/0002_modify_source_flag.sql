-- Migration number: 0002 	 2025-04-24T16:26:11.273Z

ALTER TABLE status
DROP COLUMN source; 

ALTER TABLE status
ADD seenOnJetstream bool;
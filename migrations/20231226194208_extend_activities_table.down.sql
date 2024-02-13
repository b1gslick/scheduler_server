-- Add down migration script here
ALTER TABLE activities
DROP COLUMN account_id;

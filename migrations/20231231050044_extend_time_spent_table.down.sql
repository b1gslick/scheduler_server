-- Add down migration script here
ALTER TABLE time_spent
DROP COLUMN account_id;

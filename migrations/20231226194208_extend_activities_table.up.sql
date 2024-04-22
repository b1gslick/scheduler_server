-- Add up migration script here
ALTER TABLE activities
ADD COLUMN account_id serial;

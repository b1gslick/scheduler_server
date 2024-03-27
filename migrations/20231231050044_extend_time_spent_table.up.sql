-- Add up migration script here
ALTER TABLE time_spent
ADD COLUMN account_id serial;

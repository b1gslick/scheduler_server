-- Add up migration script here
CREATE TABLE IF NOT EXISTS time_spent (
    id serial PRIMARY KEY,
    time integer NOT NULL,
    created_on TIMESTAMP NOT NULL DEFAULT NOW(),
    activity_id integer REFERENCES activities
);

-- Add up migration script here
CREATE TABLE IF NOT EXISTS activities (
    id serial PRIMARY KEY,
    title VARCHAR (255) NOT NULL,
    content TEXT NOT NULL,
    time integer NOT NULL,
    created_on TIMESTAMP NOT NULL DEFAULT NOW()
);

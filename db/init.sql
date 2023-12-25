CREATE TABLE IF NOT EXISTS activities (
    id serial PRIMARY KEY,
    title VARCHAR (255) NOT NULL,
    content TEXT NOT NULL,
    time integer NOT NULL,
    created_on TIMESTAMP NOT NULL DEFAULT NOW()
);
CREATE TABLE IF NOT EXISTS time_spent (
    id serial PRIMARY KEY,
    time integer NOT NULL,
    created_on TIMESTAMP NOT NULL DEFAULT NOW(),
    activity_id integer REFERENCES activities
);

-- Add migration script here
CREATE TABLE IF NOT EXISTS accounts(
    id SERIAL PRIMARY KEY,
    userdata jsonb NOT NULL,
    phone text NOT NULL,
    userid uuid NOT NULL
);
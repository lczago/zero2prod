-- Add migration script here
CREATE TABLE users(
    user_id uuid PRIMARY KEY,
    username TEXT NOT NULL unique,
    password TEXT NOT NULL
);
-- Add migration script here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    password VARCHAR(1000) NOT NULL,
    full_name VARCHAR(255) NOT NULL
);
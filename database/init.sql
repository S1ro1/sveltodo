CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(64) NOT NULL UNIQUE,
    password VARCHAR(512) NOT NULL
);

INSERT INTO users (name, password) VALUES ('admin', 'admin');


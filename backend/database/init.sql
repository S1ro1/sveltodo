CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(64) NOT NULL UNIQUE,
    password VARCHAR(512) NOT NULL
);

CREATE TABLE IF NOT EXISTS tasks (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    title VARCHAR(64) NOT NULL,
    description VARCHAR(512) NOT NULL,
    difficulty INTEGER NOT NULL,
    finished BOOLEAN NOT NULL DEFAULT FALSE,
    removed BOOLEAN NOT NULL DEFAULT FALSE
);

INSERT INTO users (name, password) VALUES ('admin', 'admin');

CREATE TABLE IF NOT EXISTS credentials (
    id INTEGER PRIMARY KEY NOT NULL,
    timestamp INTEGER NOT NULL,
    email TEXT NOT NULL,
    password TEXT NOT NULL
);
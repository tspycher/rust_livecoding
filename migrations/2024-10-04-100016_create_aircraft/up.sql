-- Your SQL goes here
CREATE TABLE aircraft (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    display_name TEXT NOT NULL,
    registration TEXT NOT NULL UNIQUE
);

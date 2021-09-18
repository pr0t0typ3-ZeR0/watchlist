-- Your SQL goes here
CREATE TABLE IF NOT EXISTS movies
(
    id VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    genre VARCHAR NOT NULL,
    plot VARCHAR NOT NULL,
    poster VARCHAR NOT NULL,
    runtime VARCHAR NOT NULL,
    PRIMARY KEY (id)
)

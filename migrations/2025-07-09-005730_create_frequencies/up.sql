-- Your SQL goes here
CREATE TABLE IF NOT EXISTS frequencies (
  id          INTEGER PRIMARY KEY AUTOINCREMENT,
  word        TEXT NOT NULL,
  frequency   BIGINT NOT NULL,
  lang        TEXT NOT NULL
);
-- Your SQL goes here
CREATE TABLE IF NOT EXISTS sentence_rankings (
  id          INTEGER PRIMARY KEY AUTOINCREMENT,
  sentence    TEXT NOT NULL,
  ranking     BIGINT NOT NULL,
  lang        TEXT NOT NULL
);
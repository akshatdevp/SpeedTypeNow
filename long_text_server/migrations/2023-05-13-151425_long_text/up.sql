-- Your SQL goes here
CREATE TABLE long_text (
  id SERIAL PRIMARY KEY,
  difficulty VARCHAR NOT NULL,
  body TEXT NOT NULL,
  source VARCHAR NOT NULL DEFAULT FALSE
)

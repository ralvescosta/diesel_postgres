-- Your SQL goes here
CREATE TABLE posts (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  bofy TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f'
);
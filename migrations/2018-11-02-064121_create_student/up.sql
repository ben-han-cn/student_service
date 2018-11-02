-- Your SQL goes here
CREATE TABLE students (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  age INTEGER,
  hobby TEXT,
  graduated BOOLEAN NOT NULL DEFAULT 'f'
)

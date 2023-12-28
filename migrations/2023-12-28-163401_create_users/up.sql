CREATE TABLE users(
  id SERIAL PRIMARY KEY NOT NULL,
  username varchar(64) NOT NULL UNIQUE,
  password varchar(128) NOT NULL,
  created_at TIMESTAMP DEFAULT NOW() NOT NULL
)
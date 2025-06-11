-- Add migration script here
CREATE TABLE releases (
  id BIGSERIAL PRIMARY KEY,
  repo_url VARCHAR(255) NOT NULL,
  revision VARCHAR(255) NOT NULL,
  path VARCHAR(255) NOT NULL,
  public_key TEXT NOT NULL,
  fingerprint VARCHAR(255) NOT NULL
);

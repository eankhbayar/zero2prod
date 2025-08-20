-- migrations/20250820042737_create_subscriptions_table.sql

CREATE TABLE subscriptions (
  id uuid NOT NULL,
  PRIMARY KEY (id),
  email TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL,
  subscribed_at TIMESTAMP NOT NULL
);

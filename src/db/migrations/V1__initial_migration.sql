CREATE TABLE users (
    username VARCHAR UNIQUE PRIMARY KEY NOT NULL,
    password VARCHAR NOT NULL,
    refresh_time INTERVAL NOT NULL
);

CREATE TABLE searches (
    id SERIAL UNIQUE PRIMARY KEY NOT NULL,
    username VARCHAR NOT NULL,
    subreddit VARCHAR NOT NULL,
    search_term VARCHAR NOT NULL,
    refresh_time INTERVAL,
    CONSTRAINT unique_keys UNIQUE (username, subreddit, search_term),
    FOREIGN KEY (username) REFERENCES users(username)
);

CREATE TABLE results (
    id VARCHAR NOT NULL,
    inserted TIMESTAMP NOT NULL,
    search_id INTEGER NOT NULL,
    title VARCHAR NOT NULL,
    permalink VARCHAR NOT NULL,
    thumbnail VARCHAR,
    FOREIGN KEY (search_id) REFERENCES searches(id),
    PRIMARY KEY (id, search_id)
);

CREATE TABLE gotify_settings (
  username VARCHAR NOT NULL,
  enabled BOOLEAN NOT NULL,
  server_url VARCHAR NOT NULL,
  token VARCHAR NOT NULL,
  priority BIGINT,
  FOREIGN KEY (username) REFERENCES users(username)
);

CREATE TYPE urgency AS ENUM ('info', 'success', 'warning', 'failure');

CREATE TABLE notifier_configs (
  id SERIAL UNIQUE PRIMARY KEY NOT NULL,
  username VARCHAR NOT NULL,
  name VARCHAR NOT NULL,
  uri VARCHAR NOT NULL,
  urgency URGENCY NOT NULL
);

CREATE OR REPLACE FUNCTION new_results()
RETURNS trigger AS $$
BEGIN
  PERFORM pg_notify(
    'new_results',
    row_to_json(NEW)::text
  );

  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER new_results
AFTER INSERT
ON results
FOR EACH ROW
EXECUTE PROCEDURE new_results();

CREATE OR REPLACE FUNCTION notify_search_changes()
RETURNS trigger AS $$
BEGIN
  IF TG_OP = 'DELETE' THEN
    PERFORM pg_notify(
      'searches_changes',
      json_build_object(
        'operation', TG_OP,
        'record', (row_to_json(OLD)::jsonb) || (json_build_object(
        'refresh_time', EXTRACT(epoch from OLD.refresh_time) || 's'
      )::jsonb)
    )::text);

      RETURN OLD;
  ELSE
    PERFORM pg_notify(
      'searches_changes',
      json_build_object(
        'operation', TG_OP,
        'record', (row_to_json(NEW)::jsonb) || (json_build_object(
          'refresh_time', EXTRACT(epoch from NEW.refresh_time) || 's'
        )::jsonb
      )
    )::text);

    RETURN NEW;
  END IF;

END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER searches_changes
AFTER INSERT OR UPDATE OR DELETE
ON searches
FOR EACH ROW
EXECUTE PROCEDURE notify_search_changes();
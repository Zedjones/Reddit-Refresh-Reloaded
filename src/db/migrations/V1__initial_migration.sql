CREATE TABLE users (
    username VARCHAR UNIQUE PRIMARY KEY NOT NULL,
    password VARCHAR NOT NULL,
    refresh_time TIME
);

CREATE TABLE searches (
    id SERIAL UNIQUE PRIMARY KEY NOT NULL,
    username VARCHAR NOT NULL,
    subreddit VARCHAR NOT NULL,
    search_term VARCHAR NOT NULL,
    CONSTRAINT unique_keys UNIQUE (username, subreddit, search_term),
    FOREIGN KEY (username) REFERENCES users(username)
);

CREATE TABLE results (
    id INTEGER UNIQUE PRIMARY KEY NOT NULL,
    inserted TIMESTAMP NOT NULL,
    search_id INTEGER NOT NULL,
    title VARCHAR NOT NULL,
    FOREIGN KEY (search_id) REFERENCES searches(id)
);

CREATE OR REPLACE FUNCTION notify_result_changes()
RETURNS trigger AS $$
BEGIN
  PERFORM pg_notify(
    'results_changes',
    json_build_object(
      'operation', TG_OP,
      'record', row_to_json(NEW)
    )::text
  );

  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER results_changes
AFTER INSERT OR UPDATE OR DELETE
ON results
FOR EACH ROW
EXECUTE PROCEDURE notify_result_changes();
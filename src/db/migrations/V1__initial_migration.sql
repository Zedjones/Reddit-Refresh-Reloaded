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
    id SERIAL UNIQUE PRIMARY KEY NOT NULL,
    inserted TIMESTAMP NOT NULL,
    search_id INTEGER NOT NULL,
    title VARCHAR NOT NULL,
    FOREIGN KEY (search_id) REFERENCES searches(id)
);
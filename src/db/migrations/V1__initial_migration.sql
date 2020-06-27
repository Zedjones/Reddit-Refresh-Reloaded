CREATE TABLE users (
    token VARCHAR,
    username VARCHAR UNIQUE PRIMARY KEY NOT NULL,
    password VARCHAR NOT NULL,
    refresh_time INTERVAL
);

CREATE TABLE searches (
    id SERIAL UNIQUE PRIMARY KEY NOT NULL,
    username VARCHAR NOT NULL,
    subreddit VARCHAR NOT NULL,
    search_term VARCHAR NOT NULL,
    FOREIGN KEY (username) REFERENCES users(username)
);

CREATE TABLE results (
    id SERIAL UNIQUE PRIMARY KEY NOT NULL,
    inserted DATE NOT NULL,
    search_id INTEGER NOT NULL,
    title VARCHAR NOT NULL,
    FOREIGN KEY (search_id) REFERENCES searches(id)
);
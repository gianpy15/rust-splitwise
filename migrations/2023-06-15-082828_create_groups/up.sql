-- Your SQL goes here
CREATE TABLE split_group (
  id INTEGER PRIMARY KEY NOT NULL,
  name VARCHAR NOT NULL UNIQUE
);

CREATE TABLE group_to_user (
    id INTEGER PRIMARY KEY NOT NULL,
    group_id INTEGER NOT NULL REFERENCES split_group(id),
    user_id INTEGER NOT NULL REFERENCES user(id),
    UNIQUE (group_id, user_id)
);

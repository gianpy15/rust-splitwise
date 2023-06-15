-- Your SQL goes here
CREATE TABLE split_group (
  id INTEGER PRIMARY KEY NOT NULL,
  name VARCHAR NOT NULL UNIQUE
);

CREATE TABLE group_to_user (
    group_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    PRIMARY KEY (group_id, user_id)
);

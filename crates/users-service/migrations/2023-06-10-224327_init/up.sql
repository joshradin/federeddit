-- Your SQL goes here

CREATE TABLE user (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    email VARCHAR(255) NOT NULL,
    username VARCHAR(64) NOT NULL,
    password_hash TEXT NOT NULL,

    CONSTRAINT UNIQUE INDEX (email),
    CONSTRAINT UNIQUE INDEX (username)
)
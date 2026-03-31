CREATE TABLE users
(
    id       UUID PRIMARY KEY,
    name     VARCHAR(128) CHECK ( char_length(name) >= 8 ) NOT NULL,
    email    VARCHAR(256) UNIQUE                          NOT NULL,
    password VARCHAR(128)                                 NOT NULL
)

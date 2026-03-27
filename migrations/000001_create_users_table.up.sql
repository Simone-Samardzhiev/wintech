CREATE TABLE users
(
    id       UUID PRIMARY KEY,
    name     VARCHAR(128) CHECK ( char_length(name) > 8 ),
    email    VARCHAR(256) UNIQUE,
    password VARCHAR(128)
)
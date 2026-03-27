CREATE TYPE token_type AS ENUM ('access', 'refresh');

CREATE TABLE tokens
(
    id      UUID PRIMARY KEY,
    type    token_type  NOT NULL,
    expiry  timestamptz NOT NULL,
    user_id UUID        NOT NULL REFERENCES users (id)
);
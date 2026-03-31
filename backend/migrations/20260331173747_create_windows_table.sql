CREATE TABLE windows
(
    id                UUID PRIMARY KEY,
    preferred_temp    SMALLINT NOT NULL CHECK ( preferred_temp BETWEEN -5 AND 30),
    preferred_wake_up TIME     NOT NULL,
    preferred_bedtime TIME     NOT NULL,
    user_id           UUID     NOT NULL REFERENCES users (id)
);
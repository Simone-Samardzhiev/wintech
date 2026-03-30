CREATE TABLE windows
(
    id                UUID PRIMARY KEY,
    preferred_temp    SMALLINT NOT NULL CHECK ( preferred_temp BETWEEN -5 AND 30),
    preferred_wake_up TIME     NOT NULL,
    preferred_bedtime TIME     NOT NULL,
    user_id           UUID     NOT NULL REFERENCES users (id)
);

CREATE TYPE window_action_type AS ENUM ('close', 'open');

CREATE TABLE window_actions
(
    id          UUID PRIMARY KEY,
    window_id   UUID               NOT NULL REFERENCES windows (id),
    action      window_action_type NOT NULL,
    occurred_at TIMESTAMPTZ        NOT NULL
)
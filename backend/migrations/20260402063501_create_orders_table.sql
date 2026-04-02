CREATE TABLE orders
(
    id           UUID PRIMARY KEY,
    postal_code  VARCHAR(12)  NOT NULL CHECK ( char_length(postal_code) > 3 ),
    city         VARCHAR(100) NOT NULL CHECK ( char_length(city) > 3 ),
    neighborhood VARCHAR(100) CHECK ( char_length(neighborhood) > 3 ),
    street       VARCHAR(100) NOT NULL CHECK ( char_length(street) > 3 ),
    floor_level  SMALLINT CHECK ( floor_level >= 0 ),
    created_at   TIMESTAMPTZ  NOT NULL,
    delivered_at TIMESTAMPTZ CHECK ( delivered_at > created_at ),
    user_id      UUID         NOT NULL REFERENCES users (id)
)
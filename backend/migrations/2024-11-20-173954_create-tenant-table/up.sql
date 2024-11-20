CREATE TABLE tenants (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    age INTEGER,
    image VARCHAR,
    burn_count INTEGER DEFAULT 0,
    dishwasher_count INTEGER DEFAULT 0,
    favorite_quote TEXT
);

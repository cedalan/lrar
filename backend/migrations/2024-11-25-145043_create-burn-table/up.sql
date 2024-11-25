CREATE TABLE burn (
    id SERIAL PRIMARY KEY,
    reason TEXT NOT NULL,
    receiver_id INTEGER NOT NULL REFERENCES tenants(id),
    giver_id INTEGER NOT NULL REFERENCES tenants(id),
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);


CREATE TABLE keypairs (
    id SERIAL PRIMARY KEY,
    private_key TEXT NOT NULL,
    public_key TEXT NOT NULL
);
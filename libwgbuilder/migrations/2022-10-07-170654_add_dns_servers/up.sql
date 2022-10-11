CREATE TABLE dns_servers (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    ip TEXT NOT NULL,
    description TEXT
);
CREATE TABLE tokens (
    id serial PRIMARY KEY,
    token text NOT NULL,
    expires_at timestamp
);

CREATE TABLE tokens_clients (
    token_id integer NOT NULL REFERENCES tokens(id),
    client_id integer NOT NULL REFERENCES clients(id),
    PRIMARY KEY (token_id, client_id)
);

CREATE TABLE tokens_servers (
    token_id integer NOT NULL REFERENCES tokens(id),
    server_id integer NOT NULL REFERENCES servers(id),
    PRIMARY KEY (token_id, server_id)
);
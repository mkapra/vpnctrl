CREATE TABLE clients (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    description VARCHAR(255) NULL,
    dns_server_id INTEGER NOT NULL,
    keepalive INTEGER NOT NULL DEFAULT 25,
    keypair_id INTEGER NOT NULL UNIQUE,
    vpn_ip_id INTEGER NOT NULL UNIQUE,

    FOREIGN KEY (dns_server_id) REFERENCES dns_servers(id),
    FOREIGN KEY (keypair_id) REFERENCES keypairs(id) ON DELETE CASCADE,
    FOREIGN KEY (vpn_ip_id) REFERENCES vpn_ips(id) ON DELETE CASCADE
);
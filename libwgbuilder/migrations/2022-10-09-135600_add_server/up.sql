CREATE TABLE servers(
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    forward_interface TEXT,
    keypair_id INTEGER NOT NULL,
    vpn_ip_id INTEGER NOT NULL,

    FOREIGN KEY (keypair_id) REFERENCES keypairs(id) ON DELETE CASCADE,
    FOREIGN KEY (vpn_ip_id) REFERENCES vpn_ips(id) ON DELETE CASCADE
)
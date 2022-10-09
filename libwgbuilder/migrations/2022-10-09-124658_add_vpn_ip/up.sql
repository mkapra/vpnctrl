CREATE TABLE vpn_ips (
    id SERIAL PRIMARY KEY,
    address VARCHAR NOT NULL,
    vpn_network_id INT NOT NULL,

    UNIQUE (address, vpn_network_id),
    FOREIGN KEY (vpn_network_id) REFERENCES vpn_networks(id) ON DELETE CASCADE
)
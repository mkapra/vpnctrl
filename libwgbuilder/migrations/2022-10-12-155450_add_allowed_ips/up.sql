CREATE TABLE allowed_ips (
    id SERIAL PRIMARY KEY,
    address VARCHAR(255) NOT NULL
);

CREATE TABLE allowed_ips_clients (
    id SERIAL PRIMARY KEY,
    allowed_ip_id INT NOT NULL,
    client_id INT NOT NULL,
    FOREIGN KEY (allowed_ip_id) REFERENCES allowed_ips(id),
    FOREIGN KEY (client_id) REFERENCES clients(id)
);
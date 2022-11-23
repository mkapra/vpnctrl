CREATE TABLE allowed_ips_clients_sending (
    id SERIAL PRIMARY KEY,
    allowed_ip_id INT NOT NULL,
    client_id INT NOT NULL,
    FOREIGN KEY (allowed_ip_id) REFERENCES allowed_ips(id),
    FOREIGN KEY (client_id) REFERENCES clients(id)
);

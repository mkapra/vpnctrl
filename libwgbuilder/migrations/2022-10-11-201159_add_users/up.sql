CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    role INT NOT NULL
);

INSERT INTO users (username, password, role) VALUES ('admin', '$2y$08$eZkpydtH912IZxraXhVc9.Bj5VTwFgdEupBX/IN9efdADA3w0cUyq', 0);
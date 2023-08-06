-- Add up migration script here
-- install uuid plugin
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS users (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    
    email VARCHAR(500) NOT NULL UNIQUE,
    password VARCHAR(500) NOT NULL
);

CREATE TABLE IF NOT EXISTS roles (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,

    name VARCHAR(255) NOT NULL UNIQUE,
    description VARCHAR(500) NOT NULL
);

INSERT INTO roles (name, description)
VALUES
    ('admin', 'god mode, you can do anything'),
    ('default', 'A normal user with rights to the application');

CREATE TABLE IF NOT EXISTS user_roles (
    user_id UUID,
    role_id UUID,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (role_id) REFERENCES roles(id),
    UNIQUE (user_id, role_id)
);
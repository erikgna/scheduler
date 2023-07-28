CREATE TABLE users
(
    id SERIAL PRIMARY KEY,
    email VARCHAR NOT NULL,   
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    phone VARCHAR(20) NOT NULL,
    address VARCHAR(255) NOT NULL,
    address_number VARCHAR(10) NOT NULL,
    city VARCHAR(100) NOT NULL,
    state VARCHAR(100) NOT NULL,    
    password VARCHAR NOT NULL,
    role INTEGER NOT NULL DEFAULT 0,
    photo VARCHAR(255),
    token VARCHAR
)
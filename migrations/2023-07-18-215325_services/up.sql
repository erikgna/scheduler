CREATE TABLE services (
    id_service SERIAL PRIMARY KEY,
    id_professional INTEGER REFERENCES professionals (id_professional),
    service_name VARCHAR(100) NOT NULL,
    description TEXT,
    images TEXT,
    price DECIMAL(10, 2) NOT NULL,
    duration INT NOT NULL
);
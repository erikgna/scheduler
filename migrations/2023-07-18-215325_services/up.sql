CREATE TABLE services (
    id_service SERIAL PRIMARY KEY,
    service_name VARCHAR(100) NOT NULL,
    description TEXT,
    images JSONB,
    price DECIMAL(10, 2) NOT NULL
);
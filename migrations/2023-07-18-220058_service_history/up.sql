CREATE TABLE service_history (
    id_record SERIAL PRIMARY KEY,
    id_user INTEGER REFERENCES users (id),
    id_service INTEGER REFERENCES services (id_service),
    date_time_service TIMESTAMP NOT NULL,
    amount_paid DECIMAL(10, 2) NOT NULL
);
CREATE TABLE appointments (
    id_appointment SERIAL PRIMARY KEY,
    id_user INTEGER REFERENCES users (id),
    id_professional INTEGER REFERENCES professionals (id_professional),
    id_service INTEGER REFERENCES services (id_service),
    date_time_appointment TIMESTAMP NOT NULL,
    appointment_status VARCHAR(50) NOT NULL
);
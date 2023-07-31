CREATE TABLE reviews (
    id_review SERIAL PRIMARY KEY,
    id_user INTEGER REFERENCES users (id),
    id_professional INTEGER REFERENCES professionals (id_professional),
    id_service INTEGER REFERENCES services (id_service),
    comment TEXT,
    rating INTEGER CHECK (rating >= 1 AND rating <= 5)
);
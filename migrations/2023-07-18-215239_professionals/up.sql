CREATE TABLE professionals (
    id_professional SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    specialization VARCHAR(100) NOT NULL,
    description TEXT,
    schedules TEXT,
    photo_path VARCHAR(255)
);
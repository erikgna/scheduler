CREATE TABLE professionals (
    id_professional SERIAL PRIMARY KEY,
    id_user INTEGER NOT NULL,
    specialization VARCHAR(100) NOT NULL,
    description TEXT,
    schedules TEXT
);
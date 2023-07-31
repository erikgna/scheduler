CREATE TABLE notifications (
    id_notification SERIAL PRIMARY KEY,
    id_user INTEGER REFERENCES users (id),
    id_professional INTEGER REFERENCES professionals (id_professional),
    message TEXT NOT NULL,
    date_time_sent TIMESTAMP NOT NULL
);

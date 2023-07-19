CREATE TABLE notifications (
    id_notification SERIAL PRIMARY KEY,
    id_user INTEGER REFERENCES users (id),
    message TEXT NOT NULL,
    date_time_sent TIMESTAMP NOT NULL
);

CREATE TABLE users
(
    id         SERIAL PRIMARY KEY,
    email   VARCHAR NOT NULL,    
    first_name   VARCHAR NOT NULL,
    last_name   VARCHAR NOT NULL,
    password   VARCHAR NOT NULL,
    token   VARCHAR NOT NULL
)
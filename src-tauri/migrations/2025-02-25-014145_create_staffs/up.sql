-- Your SQL goes here
CREATE TABLE staffs (
                        id SERIAL PRIMARY KEY,
                        name VARCHAR NOT NULL,
                        password VARCHAR NOT NULL,
                        admin BOOLEAN NOT NULL DEFAULT FALSE
)
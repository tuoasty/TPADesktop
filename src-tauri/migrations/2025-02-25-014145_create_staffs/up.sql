-- noinspection SqlNoDataSourceInspectionForFile

-- Your SQL goes here
CREATE TABLE staffs (
                        id SERIAL PRIMARY KEY,
                        name VARCHAR NOT NULL,
                        password VARCHAR NOT NULL,
                        role VARCHAR NOT NULL
)
-- noinspection SqlNoDataSourceInspectionForFile

-- Your SQL goes here
CREATE TABLE staffs
(
    id       SERIAL PRIMARY KEY,
    name     VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    role     VARCHAR NOT NULL
);

CREATE TABLE images(
    id SERIAL PRIMARY KEY,
    image_data BYTEA NOT NULL,
    mime_type VARCHAR NOT NULL,
    filename VARCHAR NOT NULL
);

CREATE TABLE restaurants
(
    id   SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    image_id INTEGER NOT NULL REFERENCES images(id),
    open_time TIME NOT NULL,
    close_time TIME NOT NULL,
    cuisine VARCHAR NOT NULL
);

CREATE TABLE menus (
    id SERIAL PRIMARY KEY,
    restaurant_id INTEGER NOT NULL REFERENCES restaurants(id) ON DELETE CASCADE,
    image_id INTEGER NOT NULL REFERENCES images(id),
    name VARCHAR NOT NULL,
    price INTEGER NOT NULL
);
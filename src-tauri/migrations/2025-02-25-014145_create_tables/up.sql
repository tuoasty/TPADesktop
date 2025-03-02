-- noinspection SqlNoDataSourceInspectionForFile

-- Your SQL goes here
CREATE TABLE staffs
(
    id       SERIAL PRIMARY KEY,
    name     VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    role     VARCHAR NOT NULL
);

CREATE TABLE customers
(
    id   SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL
);

CREATE TABLE images
(
    id         SERIAL PRIMARY KEY,
    image_data BYTEA   NOT NULL,
    mime_type  VARCHAR NOT NULL,
    filename   VARCHAR NOT NULL
);

CREATE TABLE restaurants
(
    id         SERIAL PRIMARY KEY,
    name       VARCHAR NOT NULL,
    image_id   INTEGER NOT NULL REFERENCES images (id) ON DELETE CASCADE,
    open_time  TIME    NOT NULL,
    close_time TIME    NOT NULL,
    cuisine    VARCHAR NOT NULL,
    status     VARCHAR NOT NULL
);

CREATE TABLE menus
(
    id            SERIAL PRIMARY KEY,
    restaurant_id INTEGER NOT NULL REFERENCES restaurants (id) ON DELETE CASCADE,
    image_id      INTEGER NOT NULL REFERENCES images (id),
    name          VARCHAR NOT NULL,
    price         INTEGER NOT NULL
);

CREATE TABLE stores
(
    id         SERIAL PRIMARY KEY,
    image_id   INTEGER NOT NULL REFERENCES images (id),
    name       VARCHAR NOT NULL,
    open_time  TIME    NOT NULL,
    close_time TIME    NOT NULL,
    status     VARCHAR NOT NULL
);

CREATE TABLE souvenirs
(
    id          SERIAL PRIMARY KEY,
    store_id    INTEGER NOT NULL REFERENCES stores (id) ON DELETE CASCADE,
    image_id    INTEGER NOT NULL REFERENCES images (id),
    name        VARCHAR NOT NULL,
    price       INTEGER NOT NULL,
    description VARCHAR NOT NULL
);

CREATE TABLE rides
(
    id         SERIAL PRIMARY KEY,
    image_id   INTEGER NOT NULL REFERENCES images (id),
    name       VARCHAR NOT NULL,
    open_time  TIME    NOT NULL,
    close_time TIME    NOT NULL,
    price      INTEGER NOT NULL,
    status     VARCHAR NOT NULL
);

CREATE TABLE ride_queues
(
    id          SERIAL PRIMARY KEY,
    ride_id     INTEGER NOT NULL REFERENCES rides (id),
    customer_id INTEGER NOT NULL REFERENCES customers (id),
    time_joined TIME    NOT NULL,
    status      VARCHAR NOT NULL
);

CREATE TABLE maintenance_reports
(
    id          SERIAL PRIMARY KEY,
    ride_id     INTEGER NOT NULL REFERENCES rides (id),
    description VARCHAR NOT NULL,
    status      VARCHAR NOT NULL
);

CREATE TABLE maintenance_assignments
(
    id                    SERIAL PRIMARY KEY,
    staff_id              INTEGER NOT NULL REFERENCES staffs (id) ON DELETE CASCADE,
    maintenance_report_id INTEGER NOT NULL REFERENCES maintenance_reports (id) ON DELETE CASCADE,
    assignment_date       TIMESTAMP WITHOUT TIME ZONE    NOT NULL,
    status                VARCHAR NOT NULL
);

CREATE TABLE store_assignments
(
    id                    SERIAL PRIMARY KEY,
    staff_id              INTEGER NOT NULL REFERENCES staffs (id) ON DELETE CASCADE,
    store_id INTEGER NOT NULL REFERENCES stores (id) ON DELETE CASCADE
);

CREATE TABLE restaurant_assignments
(
    id                    SERIAL PRIMARY KEY,
    staff_id              INTEGER NOT NULL REFERENCES staffs (id) ON DELETE CASCADE,
    restaurant_id INTEGER NOT NULL REFERENCES restaurants (id) ON DELETE CASCADE
);

CREATE TABLE ride_assignments
(
    id                    SERIAL PRIMARY KEY,
    staff_id              INTEGER NOT NULL REFERENCES staffs (id) ON DELETE CASCADE,
    ride_id INTEGER NOT NULL REFERENCES rides (id) ON DELETE CASCADE
);
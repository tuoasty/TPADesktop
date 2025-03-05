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
    id      SERIAL PRIMARY KEY,
    name    VARCHAR NOT NULL,
    balance INTEGER NOT NULL
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
    id       SERIAL PRIMARY KEY,
    staff_id INTEGER NOT NULL REFERENCES staffs (id) ON DELETE CASCADE,
    store_id INTEGER NOT NULL REFERENCES stores (id) ON DELETE CASCADE,
    role     VARCHAR NOT NULL
);

CREATE TABLE restaurant_assignments
(
    id            SERIAL PRIMARY KEY,
    staff_id      INTEGER NOT NULL REFERENCES staffs (id) ON DELETE CASCADE,
    restaurant_id INTEGER NOT NULL REFERENCES restaurants (id) ON DELETE CASCADE,
    role          VARCHAR NOT NULL
);

CREATE TABLE ride_assignments
(
    id       SERIAL PRIMARY KEY,
    staff_id INTEGER NOT NULL REFERENCES staffs (id) ON DELETE CASCADE,
    ride_id  INTEGER NOT NULL REFERENCES rides (id) ON DELETE CASCADE,
    role     VARCHAR NOT NULL
);

CREATE TABLE lost_items
(
    id             SERIAL PRIMARY KEY,
    name           VARCHAR NOT NULL,
    item_type      VARCHAR NOT NULL,
    color          VARCHAR NOT NULL,
    last_location  VARCHAR NOT NULL,
    owner_id       INTEGER NOT NULL,
    status         VARCHAR NOT NULL,
    finder_id      INTEGER,
    found_location VARCHAR,
    image_id       INTEGER REFERENCES images (id)
);

CREATE TABLE notifications
(
    id          SERIAL PRIMARY KEY,
    customer_id INTEGER NOT NULL REFERENCES customers (id),
    message     VARCHAR NOT NULL
);

CREATE TABLE ride_proposals
(
    id            SERIAL PRIMARY KEY,
    proposal_type VARCHAR NOT NULL,
    status        VARCHAR NOT NULL,
    description   VARCHAR NOT NULL,
    price INTEGER NOT NULL,
    ride_id       INTEGER REFERENCES rides (id),
    image_id      INTEGER REFERENCES images (id)
);

CREATE TABLE store_proposals
(
    id            SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    proposal_type VARCHAR NOT NULL,
    status        VARCHAR NOT NULL,
    description   VARCHAR NOT NULL,
    store_id       INTEGER REFERENCES stores (id),
    image_id      INTEGER REFERENCES images (id)
);

CREATE TABLE restaurant_proposals(
    id SERIAL PRIMARY KEY,
    image_id INTEGER NOT NULL REFERENCES images(id),
    open_time TIME NOT NULL,
    close_time TIME NOT NULL,
    cuisine VARCHAR NOT NULL
);
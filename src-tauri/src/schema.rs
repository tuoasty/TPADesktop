// @generated automatically by Diesel CLI.

diesel::table! {
    images (id) {
        id -> Int4,
        image_data -> Bytea,
        mime_type -> Varchar,
        filename -> Varchar,
    }
}

diesel::table! {
    menus (id) {
        id -> Int4,
        restaurant_id -> Int4,
        image_id -> Int4,
        name -> Varchar,
        price -> Int4,
    }
}

diesel::table! {
    restaurants (id) {
        id -> Int4,
        name -> Varchar,
        image_id -> Int4,
        open_time -> Time,
        close_time -> Time,
        cuisine -> Varchar,
    }
}

diesel::table! {
    souvenirs (id) {
        id -> Int4,
        store_id -> Int4,
        image_id -> Int4,
        name -> Varchar,
        price -> Int4,
        description -> Varchar,
    }
}

diesel::table! {
    staffs (id) {
        id -> Int4,
        name -> Varchar,
        password -> Varchar,
        role -> Varchar,
    }
}

diesel::table! {
    stores (id) {
        id -> Int4,
        image_id -> Int4,
        name -> Varchar,
        open_time -> Time,
        close_time -> Time,
    }
}

diesel::joinable!(menus -> images (image_id));
diesel::joinable!(menus -> restaurants (restaurant_id));
diesel::joinable!(restaurants -> images (image_id));
diesel::joinable!(souvenirs -> images (image_id));
diesel::joinable!(souvenirs -> stores (store_id));
diesel::joinable!(stores -> images (image_id));

diesel::allow_tables_to_appear_in_same_query!(
    images,
    menus,
    restaurants,
    souvenirs,
    staffs,
    stores,
);

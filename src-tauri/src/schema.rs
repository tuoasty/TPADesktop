// @generated automatically by Diesel CLI.

diesel::table! {
    customers (id) {
        id -> Int4,
        name -> Varchar,
        balance -> Int4,
    }
}

diesel::table! {
    images (id) {
        id -> Int4,
        image_data -> Bytea,
        mime_type -> Varchar,
        filename -> Varchar,
    }
}

diesel::table! {
    lost_items (id) {
        id -> Int4,
        name -> Varchar,
        item_type -> Varchar,
        color -> Varchar,
        last_location -> Varchar,
        owner_id -> Int4,
        status -> Varchar,
        finder_id -> Nullable<Int4>,
        found_location -> Nullable<Varchar>,
        image_id -> Nullable<Int4>,
    }
}

diesel::table! {
    maintenance_assignments (id) {
        id -> Int4,
        staff_id -> Int4,
        maintenance_report_id -> Int4,
        assignment_date -> Timestamp,
        status -> Varchar,
    }
}

diesel::table! {
    maintenance_reports (id) {
        id -> Int4,
        ride_id -> Int4,
        description -> Varchar,
        status -> Varchar,
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
    notifications (id) {
        id -> Int4,
        customer_id -> Int4,
        message -> Varchar,
    }
}

diesel::table! {
    restaurant_assignments (id) {
        id -> Int4,
        staff_id -> Int4,
        restaurant_id -> Int4,
        role -> Varchar,
    }
}

diesel::table! {
    restaurant_proposals (id) {
        id -> Int4,
        image_id -> Int4,
        open_time -> Time,
        close_time -> Time,
        cuisine -> Varchar,
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
        status -> Varchar,
    }
}

diesel::table! {
    ride_assignments (id) {
        id -> Int4,
        staff_id -> Int4,
        ride_id -> Int4,
        role -> Varchar,
    }
}

diesel::table! {
    ride_proposals (id) {
        id -> Int4,
        proposal_type -> Varchar,
        status -> Varchar,
        description -> Varchar,
        price -> Int4,
        ride_id -> Nullable<Int4>,
        image_id -> Nullable<Int4>,
    }
}

diesel::table! {
    ride_queues (id) {
        id -> Int4,
        ride_id -> Int4,
        customer_id -> Int4,
        time_joined -> Time,
        status -> Varchar,
    }
}

diesel::table! {
    rides (id) {
        id -> Int4,
        image_id -> Int4,
        name -> Varchar,
        open_time -> Time,
        close_time -> Time,
        price -> Int4,
        status -> Varchar,
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
    store_assignments (id) {
        id -> Int4,
        staff_id -> Int4,
        store_id -> Int4,
        role -> Varchar,
    }
}

diesel::table! {
    store_proposals (id) {
        id -> Int4,
        name -> Varchar,
        proposal_type -> Varchar,
        status -> Varchar,
        description -> Varchar,
        store_id -> Nullable<Int4>,
        image_id -> Nullable<Int4>,
    }
}

diesel::table! {
    stores (id) {
        id -> Int4,
        image_id -> Int4,
        name -> Varchar,
        open_time -> Time,
        close_time -> Time,
        status -> Varchar,
    }
}

diesel::joinable!(lost_items -> images (image_id));
diesel::joinable!(maintenance_assignments -> maintenance_reports (maintenance_report_id));
diesel::joinable!(maintenance_assignments -> staffs (staff_id));
diesel::joinable!(maintenance_reports -> rides (ride_id));
diesel::joinable!(menus -> images (image_id));
diesel::joinable!(menus -> restaurants (restaurant_id));
diesel::joinable!(notifications -> customers (customer_id));
diesel::joinable!(restaurant_assignments -> restaurants (restaurant_id));
diesel::joinable!(restaurant_assignments -> staffs (staff_id));
diesel::joinable!(restaurant_proposals -> images (image_id));
diesel::joinable!(restaurants -> images (image_id));
diesel::joinable!(ride_assignments -> rides (ride_id));
diesel::joinable!(ride_assignments -> staffs (staff_id));
diesel::joinable!(ride_proposals -> images (image_id));
diesel::joinable!(ride_proposals -> rides (ride_id));
diesel::joinable!(ride_queues -> customers (customer_id));
diesel::joinable!(ride_queues -> rides (ride_id));
diesel::joinable!(rides -> images (image_id));
diesel::joinable!(souvenirs -> images (image_id));
diesel::joinable!(souvenirs -> stores (store_id));
diesel::joinable!(store_assignments -> staffs (staff_id));
diesel::joinable!(store_assignments -> stores (store_id));
diesel::joinable!(store_proposals -> images (image_id));
diesel::joinable!(store_proposals -> stores (store_id));
diesel::joinable!(stores -> images (image_id));

diesel::allow_tables_to_appear_in_same_query!(
    customers,
    images,
    lost_items,
    maintenance_assignments,
    maintenance_reports,
    menus,
    notifications,
    restaurant_assignments,
    restaurant_proposals,
    restaurants,
    ride_assignments,
    ride_proposals,
    ride_queues,
    rides,
    souvenirs,
    staffs,
    store_assignments,
    store_proposals,
    stores,
);

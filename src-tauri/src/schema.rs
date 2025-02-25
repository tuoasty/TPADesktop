// @generated automatically by Diesel CLI.

diesel::table! {
    staffs (id) {
        id -> Int4,
        name -> Varchar,
        password -> Varchar,
        admin -> Bool,
    }
}

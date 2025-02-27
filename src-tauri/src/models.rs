use chrono::NaiveTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::staffs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Staff {
    pub id: i32,
    pub name: String,
    pub password: String,
    pub role: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::staffs)]
pub struct NewStaff {
    pub name: String,
    pub password: String,
    pub role: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::images)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Image {
    pub id: i32,
    pub image_data: Vec<u8>,
    pub mime_type: String,
    pub filename: String,
}
#[derive(Insertable)]
#[diesel(table_name = crate::schema::images)]
pub struct NewImage {
    pub image_data: Vec<u8>,
    pub mime_type: String,
    pub filename: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::restaurants)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Restaurant {
    pub id: i32,
    pub name: String,
    pub image_id: i32,
    pub open_time: NaiveTime,
    pub close_time: NaiveTime,
    pub cuisine: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::restaurants)]
pub struct NewRestaurant {
    pub name: String,
    pub image_id: i32,
    pub open_time: NaiveTime,
    pub close_time: NaiveTime,
    pub cuisine: String,
}

#[derive(Queryable, Selectable)]
#[diesel(belongs_to(Restaurant))]
#[diesel(table_name = crate::schema::menus)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Menu {
    pub id: i32,
    pub restaurant_id: i32,
    pub image_id: i32,
    pub name: String,
    pub price: i32,
}

#[derive(Insertable)]
#[diesel(belongs_to(Restaurant))]
#[diesel(table_name = crate::schema::menus)]
pub struct NewMenu {
    pub restaurant_id: i32,
    pub image_id: i32,
    pub name: String,
    pub price: i32,
}

#[derive(Deserialize)]
pub struct NewMenuDetail {
    pub name: String,
    pub restaurant_id: i32,
    pub price: i32,
    pub image_data: String,
    pub mime_type: String,
    pub image_name: String,
}

#[derive(Serialize)]
pub struct RestaurantDetail {
    pub id: i32,
    pub name: String,
    pub open_time: String,
    pub close_time: String,
    pub cuisine: String,
    pub image_data: String,
}

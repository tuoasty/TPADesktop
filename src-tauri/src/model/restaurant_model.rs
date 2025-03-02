use chrono::NaiveTime;
use diesel::{Insertable, Queryable, Selectable};
use serde::Serialize;
use crate::model::menu_model::MenuDetail;

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
    pub status:String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::restaurants)]
pub struct NewRestaurant {
    pub name: String,
    pub image_id: i32,
    pub open_time: NaiveTime,
    pub close_time: NaiveTime,
    pub cuisine: String,
    pub status:String,
}

#[derive(Serialize)]
pub struct RestaurantDetail {
    pub id: i32,
    pub name: String,
    pub open_time: String,
    pub close_time: String,
    pub cuisine: String,
    pub status:String,
    pub image_data: String,
    pub menus: Vec<MenuDetail>
}
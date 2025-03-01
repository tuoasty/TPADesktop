use chrono::NaiveTime;
use diesel::{Insertable, Queryable, Selectable};
use serde::Serialize;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::rides)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Ride {
    pub id: i32,
    pub image_id:i32,
    pub name:String,
    pub open_time:NaiveTime,
    pub close_time:NaiveTime,
    pub price:i32,
    pub status:String
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::rides)]
pub struct NewRide {
    pub image_id:i32,
    pub name:String,
    pub open_time:NaiveTime,
    pub close_time:NaiveTime,
    pub price:i32,
    pub status:String
}

#[derive(Serialize)]
pub struct RideDetail {
    pub id: i32,
    pub name: String,
    pub open_time: String,
    pub close_time: String,
    pub price: i32,
    pub image_data: String,
    pub status:String
}
use chrono::NaiveTime;
use diesel::{Insertable, Queryable, Selectable};
use serde::Serialize;
use crate::model::souvenir_model::SouvenirDetail;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::stores)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Store {
    pub id: i32,
    pub name: String,
    pub image_id: i32,
    pub open_time: NaiveTime,
    pub close_time: NaiveTime,
    pub status:String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::stores)]
pub struct NewStore {
    pub name: String,
    pub image_id: i32,
    pub open_time: NaiveTime,
    pub close_time: NaiveTime,
    pub status:String,
}

#[derive(Serialize)]
pub struct StoreDetail {
    pub id: i32,
    pub name: String,
    pub open_time: String,
    pub close_time: String,
    pub status:String,
    pub image_data: String,
    pub souvenirs: Vec<SouvenirDetail>
}
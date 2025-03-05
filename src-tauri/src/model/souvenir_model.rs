use diesel::{Insertable, Queryable, Selectable};
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(belongs_to(Store, foreign_key = store_id))]
#[diesel(table_name = crate::schema::souvenirs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Souvenir {
    pub id: i32,
    pub store_id: i32,
    pub image_id: i32,
    pub name: String,
    pub price: i32,
    pub description: String
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::souvenirs)]
pub struct NewSouvenir {
    pub name: String,
    pub store_id: i32,
    pub image_id: i32,
    pub price: i32,
    pub description: String
}

#[derive(Serialize)]
pub struct SouvenirDetail {
    pub id: i32,
    pub name: String,
    pub price: i32,
    pub description: String,
    pub image_data: String
}

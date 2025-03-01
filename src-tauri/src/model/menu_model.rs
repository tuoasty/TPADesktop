use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable)]
#[diesel(belongs_to(Restaurant, foreign_key = restaurant_id))]
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
pub struct MenuDetail {
    pub id:i32,
    pub name: String,
    pub price: i32,
    pub image_data: String
}
use diesel::{Insertable, Queryable, Selectable};

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
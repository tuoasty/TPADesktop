use diesel::{Insertable, Queryable, Selectable};
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(belongs_to(Customer, foreign_key = customer_id))]
#[diesel(table_name = crate::schema::notifications)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Notification{
    pub id:i32,
    pub customer_id:i32,
    pub message:String
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::notifications)]
pub struct NewNotification{
    pub customer_id:i32,
    pub message:String
}
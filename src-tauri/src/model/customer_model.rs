use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::customers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Customer {
    pub id: i32,
    pub name: String,
    pub balance: i32,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::customers)]
pub struct NewCustomer {
    pub name: String,
    pub balance: i32,
}
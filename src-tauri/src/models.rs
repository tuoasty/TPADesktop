use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::staffs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Staff {
    pub id : i32,
    pub name : String,
    pub password : String,
    pub role : String
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::staffs)]
pub struct NewStaff {
    pub name: String,
    pub password: String,
    pub role: String,
}
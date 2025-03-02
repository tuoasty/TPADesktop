use diesel::{Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable)]
#[diesel(belongs_to(Store, foreign_key = store_id))]
#[diesel(belongs_to(Staff, foreign_key = staff_id))]
#[diesel(table_name = crate::schema::store_assignments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StoreAssignment {
    pub id: i32,
    pub staff_id: i32,
    pub store_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::store_assignments)]
pub struct NewStoreAssignment {
    pub staff_id: i32,
    pub store_id: i32,
}
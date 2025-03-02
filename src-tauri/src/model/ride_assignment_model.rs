use diesel::{Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable)]
#[diesel(belongs_to(Ride, foreign_key = ride_id))]
#[diesel(belongs_to(Staff, foreign_key = staff_id))]
#[diesel(table_name = crate::schema::ride_assignments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RideAssignment {
    pub id: i32,
    pub staff_id: i32,
    pub ride_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::ride_assignments)]
pub struct NewRideAssignment {
    pub staff_id: i32,
    pub ride_id: i32,
}
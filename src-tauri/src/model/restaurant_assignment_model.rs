use diesel::{Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable)]
#[diesel(belongs_to(Restaurant, foreign_key = restaurant_id))]
#[diesel(belongs_to(Staff, foreign_key = staff_id))]
#[diesel(table_name = crate::schema::restaurant_assignments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RestaurantAssignment {
    pub id: i32,
    pub staff_id: i32,
    pub restaurant_id: i32,
    pub role:String
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::restaurant_assignments)]
pub struct NewRestaurantAssignment {
    pub staff_id: i32,
    pub restaurant_id: i32,
    pub role:String
}
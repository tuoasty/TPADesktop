use chrono::NaiveTime;
use diesel::{Insertable, Queryable, Selectable};
use serde::Serialize;

#[derive(Queryable, Selectable)]
#[diesel(belongs_to(Customer, foreign_key = customer_id))]
#[diesel(belongs_to(Restaurant, foreign_key = restaurant_id))]
#[diesel(belongs_to(Menu, foreign_key = menu_id))]
#[diesel(table_name = crate::schema::restaurant_orders)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RestaurantOrder {
    pub id:i32,
    pub customer_id:i32,
    pub restaurant_id:i32,
    pub menu_id:i32,
    pub status:String,
    pub count:i32,
    pub value:i32,
    pub time_ordered:NaiveTime
}

#[derive(Insertable, Serialize)]
#[diesel(table_name = crate::schema::restaurant_orders)]
pub struct RestaurantOrderDetail {
    pub id:i32,
    pub customer_id:i32,
    pub restaurant_id:i32,
    pub menu_id:i32,
    pub status:String,
    pub count:i32,
    pub value:i32,
}


#[derive(Insertable)]
#[diesel(table_name = crate::schema::restaurant_orders)]
pub struct NewRestaurantOrder {
    pub customer_id:i32,
    pub restaurant_id:i32,
    pub menu_id:i32,
    pub status:String,
    pub count:i32,
    pub value:i32,
    pub time_ordered:NaiveTime
}
use chrono::NaiveTime;
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::ride_queues)]
#[diesel(belongs_to(Ride, foreign_key = ride_id))]
#[diesel(belongs_to(Customer, foreign_key = customer_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RideQueue {
    pub id: i32,
    pub ride_id:i32,
    pub customer_id:i32,
    pub time_joined:NaiveTime,
    pub status:String,
    pub value:i32
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::ride_queues)]
pub struct NewRideQueue {
    pub ride_id:i32,
    pub customer_id:i32,
    pub time_joined:NaiveTime,
    pub status:String,
    pub value:i32
}

#[derive(Serialize)]
pub struct RideQueueDetail {
    pub id: i32,
    pub ride_id:i32,
    pub ride_name:String,
    pub customer_id:i32,
    pub customer_name:String,
    pub time_joined:String,
    pub status:String,
    pub value:i32
}

#[derive(Serialize, Deserialize)]
pub struct RideRevenue {
    pub id:i32,
    pub time:String,
    pub value:i32,
    pub ride_id:i32,
}
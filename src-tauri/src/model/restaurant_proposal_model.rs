use chrono::NaiveTime;
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::restaurant_proposals)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RestaurantProposal {
    pub id: i32,
    pub image_id:i32,
    pub open_time:NaiveTime,
    pub close_time:NaiveTime,
    pub cuisine:String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::restaurant_proposals)]
pub struct NewRestaurantProposal {
    pub image_id:i32,
    pub open_time:NaiveTime,
    pub close_time:NaiveTime,
    pub cuisine:String,
}

#[derive(Serialize)]
pub struct RestaurantProposalDetail {
    pub id: i32,
    pub open_time:String,
    pub close_time:String,
    pub cuisine:String,
    pub image_data:String,
}
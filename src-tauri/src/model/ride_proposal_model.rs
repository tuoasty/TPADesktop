use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::ride_proposals)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RideProposal {
    pub id: i32,
    pub proposal_type:String,
    pub status:String,
    pub description:String,
    pub ride_id:Option<i32>,
    pub image_id:Option<i32>
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::ride_proposals)]
pub struct NewRideProposal {
    pub proposal_type:String,
    pub status:String,
    pub description:String,
    pub ride_id:Option<i32>,
    pub image_id:Option<i32>
}

#[derive(Serialize)]
pub struct RideDetail {
    pub proposal_type:String,
    pub status:String,
    pub description:String,
    pub ride_id:Option<i32>,
    pub ride_name:Option<String>,
    pub image_data:Option<String>,
}
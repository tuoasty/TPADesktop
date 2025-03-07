use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::store_proposals)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StoreProposal {
    pub id: i32,
    pub name:String,
    pub proposal_type:String,
    pub status:String,
    pub description:String,
    pub store_id:Option<i32>,
    pub image_id:Option<i32>
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::store_proposals)]
pub struct NewStoreProposal {
    pub proposal_type:String,
    pub name:String,
    pub status:String,
    pub description:String,
    pub store_id:Option<i32>,
    pub image_id:Option<i32>
}

#[derive(Serialize)]
pub struct StoreProposalDetail {
    pub id:i32,
    pub name:String,
    pub proposal_type:String,
    pub status:String,
    pub description:String,
    pub store_id:Option<i32>,
    pub image_data:Option<String>,
}

#[derive(Deserialize)]
pub struct NewStoreProposalDetail {
    pub name:String,
    pub description:String,
    pub image_data: String,
    pub mime_type: String,
    pub image_name: String
}
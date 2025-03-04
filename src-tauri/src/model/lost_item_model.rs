use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::lost_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LostItem {
    pub id:i32,
    pub name:String,
    pub item_type:String,
    pub color:String,
    pub last_location:String,
    pub owner_id:i32,
    pub status:String,
    pub finder_id:Option<i32>,
    pub found_location:Option<String>,
    pub image_id:Option<i32>
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::lost_items)]
pub struct NewLostItem {
    pub name:String,
    pub item_type:String,
    pub color:String,
    pub last_location:String,
    pub owner_id:i32,
    pub status:String,
    pub finder_id:Option<i32>,
    pub found_location:Option<String>,
    pub image_id:Option<i32>
}

#[derive(Serialize)]
pub struct LostItemDetail {
    pub id:i32,
    pub name:String,
    pub item_type:String,
    pub color:String,
    pub last_location:String,
    pub owner_id:i32,
    pub owner_name:String,
    pub status:String,
    pub finder_id:Option<i32>,
    pub finder_name:Option<String>,
    pub found_location:Option<String>,
    pub image_data:Option<String>
}

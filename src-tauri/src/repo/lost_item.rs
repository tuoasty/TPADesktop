use diesel::QueryDsl;
use crate::DbConnect;
use crate::model::lost_item_model::{LostItem, LostItemDetail};
use crate::schema::lost_items::dsl::lost_items;
use diesel::prelude::*;
use crate::handler::image_handler::get_image_data;

impl LostItem {
    pub fn get_all_lost_item(conn: &mut DbConnect) -> Result<Vec<LostItemDetail>, String> {
        let items = lost_items.select(LostItem::as_select()).load(conn).map_err(|e| e.to_string())?;

        let lost_items_details: Vec<LostItemDetail> = items
            .into_iter()
            .map(|item| {

                let base64_image = match item.image_id {
                    Some(id) => match get_image_data(conn, id) {
                        Ok(data) => Some(data),
                        Err(_) => None
                    }
                    None => None
                };

                LostItemDetail {
                    id:item.id,
                    name:item.name,
                    item_type:item.item_type,
                    color:item.color,
                    last_location:item.last_location,
                    owner_id:item.owner_id,
                    status:item.status,
                    finder_id:item.finder_id,
                    found_location:item.found_location,
                    image_data:base64_image
                }
            })
            .collect();

        Ok(lost_items_details)
    }
}
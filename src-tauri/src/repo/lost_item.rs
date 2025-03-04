use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use diesel::QueryDsl;
use crate::DbConnect;
use crate::model::lost_item_model::{LostItem, LostItemDetail, NewLostItemDetail};
use crate::schema::lost_items::dsl::lost_items;
use diesel::prelude::*;
use crate::handler::customer_handler::find_customer_name;
use crate::handler::image_handler::{create_image, get_image_data};
use crate::handler::notification_handler::create_customer_notification;
use crate::schema::lost_items::*;

impl LostItem {
    pub fn get_all_lost_item(conn: &mut DbConnect) -> Result<Vec<LostItemDetail>, String> {
        let items =
            lost_items
                .select(LostItem::as_select())
                .load(conn)
                .map_err(|e| e.to_string())?;

        let lost_items_details: Vec<LostItemDetail> = items
            .into_iter()
            .map(|item| {

                let base64_image = match item.image_id {
                    Some(new_image_id) => match get_image_data(conn, new_image_id) {
                        Ok(data) => Some(data),
                        Err(_) => None
                    }
                    None => None
                };

                let customer_name = find_customer_name(conn, item.owner_id).unwrap();

                let finder_name = match item.finder_id {
                    Some(customer_id) => match find_customer_name(conn, customer_id) {
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
                    owner_name:customer_name,
                    status:item.status,
                    finder_id:item.finder_id,
                    finder_name:finder_name,
                    found_location:item.found_location,
                    image_data:base64_image
                }
            })
            .collect();

        Ok(lost_items_details)
    }

    pub fn update_lost_item_details(conn: &mut DbConnect, item:NewLostItemDetail) -> Result<(), String> {
        let new_image_id = match item.image_data {
            Some(image) => {
                let image_data = STANDARD.decode(image).map_err(|_| "Invalid Base encoding".to_string())?;
                Some(create_image(conn, image_data, item.mime_type.unwrap(), item.image_name.unwrap())?)
            }
            None => None
        };

        diesel::update(lost_items)
            .filter(id.eq(&item.id))
            .set((
                name.eq(&item.name),
                item_type.eq(&item.item_type),
                color.eq(&item.color),
                last_location.eq(&item.last_location),
                status.eq(&item.status),
                owner_id.eq(&item.owner_id),
                finder_id.eq(&item.finder_id),
                found_location.eq(&item.found_location),
                image_id.eq(&new_image_id)
            ))
            .execute(conn)
            .map_err(|e| e.to_string())?;

        create_customer_notification(conn, item.owner_id, format!("An item has been {}", item.status))?;

        Ok(())
    }
}
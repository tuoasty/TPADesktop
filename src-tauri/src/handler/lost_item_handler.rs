use bigdecimal::Zero;
use tauri::{command, State};
use crate::{get_conn, DbPool};
use crate::model::lost_item_model::{LostItem, LostItemDetail, NewLostItemDetail};

#[command]
pub fn find_all_lost_item(state:State<DbPool>) -> Result<Vec<LostItemDetail>, String> {
    let conn = &mut get_conn(&state)?;
    LostItem::get_all_lost_item(conn)
}

#[command]
pub fn update_lost_item(state:State<DbPool>, item:NewLostItemDetail) -> Result<(), String> {
    if item.name.is_empty() || item.item_type.is_empty() || item.status.is_empty() || item.owner_id.is_zero() ||
        item.last_location.is_empty() || item.color.is_empty() {
        return Err("All fields must be filled".to_string())
    }

    if item.status == "Found" || item.status == "Returned to Owner" {
        if item.finder_id.is_none() || item.found_location.is_none() || item.image_data.is_none() {
            return Err("All fields must be filled".to_string())
        }
    }
    let conn = &mut get_conn(&state)?;

    LostItem::update_lost_item_details(conn, item)
}
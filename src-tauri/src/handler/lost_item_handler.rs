use tauri::{command, State};
use crate::{get_conn, DbPool};
use crate::model::lost_item_model::{LostItem, LostItemDetail};

#[command]
pub fn find_all_lost_item(state:State<DbPool>) -> Result<Vec<LostItemDetail>, String> {
    let conn = &mut get_conn(&state)?;
    LostItem::get_all_lost_item(conn)
}
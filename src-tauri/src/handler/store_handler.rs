use tauri::{command, State};
use crate::{get_conn, DbPool};
use crate::model::store_model::{Store, StoreDetail};

#[command]
pub fn find_all_store(state: State<DbPool>) -> Result<Vec<StoreDetail>, String> {
    let conn = &mut get_conn(&state)?;

    let store_details: Vec<StoreDetail> = Store::get_all_stores(conn)?;

    Ok(store_details)
}

#[command]
pub fn change_store_status(state:State<DbPool>, store_id:i32, store_status:String) -> Result<(), String> {
    let conn = &mut get_conn(&state)?;

    let mut new_status = store_status.clone();

    if store_status == "Open" {
        new_status = "Closed".to_string();
    } else if store_status == "Closed" {
        new_status = "Open".to_string();
    }

    Store::update_store_status(conn, store_id, new_status)
}

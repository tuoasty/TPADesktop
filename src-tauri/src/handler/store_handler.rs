use tauri::{command, State};
use crate::{get_conn, DbPool};
use crate::models::{Store, StoreDetail};

#[command]
pub fn find_all_store(state: State<DbPool>) -> Result<Vec<StoreDetail>, String> {
    let conn = &mut get_conn(&state)?;

    let store_details: Vec<StoreDetail> = Store::get_all_stores(conn)?;

    Ok(store_details)
}



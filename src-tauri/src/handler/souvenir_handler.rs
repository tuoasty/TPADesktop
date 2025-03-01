use tauri::{command, State};
use crate::{get_conn, DbConnect, DbPool};
use crate::model::souvenir_model::{Souvenir, SouvenirDetail};
pub fn find_store_souvenir(conn: &mut DbConnect,selected_id:i32) -> Result<Vec<SouvenirDetail>, String> {
    let store_souvenirs = Souvenir::get_souvenir_of_store(conn, selected_id)?;

    Ok(store_souvenirs)
}

#[command]
pub fn remove_souvenir(state:State<DbPool>, selected_id:i32) -> Result<(), String> {
    let conn = &mut get_conn(&state)?;

    Souvenir::remove_souvenir_and_image(conn, selected_id)
}
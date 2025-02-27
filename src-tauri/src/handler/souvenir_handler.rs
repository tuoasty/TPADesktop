use diesel::RunQueryDsl;
use tauri::{command, State};
use crate::{get_conn, DbConnect, DbPool};
use crate::models::{Menu, MenuDetail, Souvenir};

pub fn find_store_souvenir(conn: &mut DbConnect, id:i32) -> Result<Vec<Souvenir>, String> {
    let store_souvenirs = Souvenir::get_souvenir_of_store(conn, id)?;

    Ok(store_souvenirs)
}

#[command]
pub fn remove_souvenir(state:State<DbPool>, souvenir_id:i32) -> Result<(), String> {
    use crate::schema::souvenirs::dsl::*;
    let conn = &mut get_conn(&state)?;

    diesel::delete(souvenirs.filter(id.eq(souvenir_id)))
        .execute(conn)
        .map_err(|e| format!("Failed to delete souvenir: {}", e))?;

    Ok(())
}
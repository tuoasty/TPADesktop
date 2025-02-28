use tauri::{command, State};
use crate::{get_conn, DbPool};
use crate::models::{Ride, RideDetail};

#[command]
pub fn find_all_ride(state:State<DbPool>) -> Result<Vec<RideDetail>, String> {
    let conn = &mut get_conn(&state)?;
    let ride_details:Vec<RideDetail> = Ride::get_all_ride(conn)?;

    Ok(ride_details)
}
use tauri::{command, State};
use crate::{get_conn, DbConnect, DbPool};
use crate::models::{Ride, RideDetail};

#[command]
pub fn find_all_ride(state:State<DbPool>) -> Result<Vec<RideDetail>, String> {
    let conn = &mut get_conn(&state)?;
    let ride_details:Vec<RideDetail> = Ride::get_all_ride(conn)?;

    Ok(ride_details)
}

#[command]
pub fn change_ride_status(state:State<DbPool>, ride_id:i32, ride_status:String) -> Result<(), String> {
    let conn = &mut get_conn(&state)?;

    let mut new_status = ride_status.clone();

    if ride_status == "Open" {
        new_status = "Closed".to_string();
    } else if ride_status == "Closed" {
        new_status = "Open".to_string();
    }

    Ride::update_ride_status(conn, ride_id, new_status)
}

pub fn find_ride(conn:&mut DbConnect, ride_id:i32) -> Result<Ride, String> {
    Ride::get_ride(conn, ride_id)
}
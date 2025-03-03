use tauri::{command, State};
use crate::{get_conn, DbConnect, DbPool};
use crate::model::ride_model::{Ride, RideDetail};

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

#[command]
pub fn reassign_ride_and_check_status(state:State<DbPool>, new_staff_id:i32, new_ride_id:i32) -> Result<(), String> {
    let conn = &mut get_conn(&state)?;

    let removed_ride_staff_id = Ride::reassign_ride_staff(conn, new_staff_id, new_ride_id)?;

    Ride::check_ride_assignment_and_update(conn, removed_ride_staff_id)
}

pub fn accept_ride_maintenance(conn: &mut DbConnect, ride_id:i32) -> Result<(), String> {
    Ride::update_ride_status(conn, ride_id, "Maintenance in Progress".to_string())
}

pub fn reject_ride_maintenance(conn: &mut DbConnect, ride_id:i32) -> Result<(), String> {
    Ride::update_ride_status(conn, ride_id, "Closed".to_string())
}

pub fn find_ride(conn:&mut DbConnect, ride_id:i32) -> Result<Ride, String> {
    Ride::get_ride(conn, ride_id)
}
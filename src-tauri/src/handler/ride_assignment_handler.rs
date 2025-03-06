use tauri::{command, State};
use crate::{get_conn, DbConnect, DbPool};
use crate::model::ride_assignment_model::RideAssignment;
use crate::model::staff_model::StaffDetail;

pub fn get_ride_staffs(conn: &mut DbConnect, selected_id:i32) -> Result<Vec<StaffDetail>, String> {
    RideAssignment::get_ride_assignment(conn, selected_id)
}

pub fn check_ride_staff_to_open(conn: &mut DbConnect, selected_id:i32) -> Result<bool, String> {
    RideAssignment::check_ride_staffing(conn, selected_id)
}

#[command]
pub fn assign_staff_to_ride(state:State<DbPool>, staff_id:i32, ride_id:i32) -> Result<(), String> {
    let conn = &mut get_conn(&state)?;
    RideAssignment::assign_ride_staff(conn, staff_id, ride_id)
}

pub fn reassign_staff_to_ride(conn: &mut DbConnect, staff_id:i32, ride_id:i32) -> Result<i32, String> {
    RideAssignment::reassign_ride_staff(conn, staff_id, ride_id)
}

pub fn find_staff_ride(conn: &mut DbConnect, selected_id:i32) -> Result<i32, String> {
    RideAssignment::get_staff_ride(conn, selected_id)
}
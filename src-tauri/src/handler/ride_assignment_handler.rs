use tauri::{command, State};
use crate::{get_conn, DbConnect, DbPool};
use crate::model::ride_assignment_model::RideAssignment;
use crate::model::staff_model::StaffDetail;

pub fn get_ride_staffs(conn: &mut DbConnect, selected_id:i32) -> Result<Vec<StaffDetail>, String> {
    RideAssignment::get_ride_assignment(conn, selected_id)
}

#[command]
pub fn assign_staff_to_ride(state:State<DbPool>, staff_id:i32, ride_id:i32) -> Result<(), String> {
    let conn = &mut get_conn(&state)?;
    RideAssignment::assign_ride_staff(conn, staff_id, ride_id)
}
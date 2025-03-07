use tauri::{command, State};
use crate::{get_conn, DbConnect, DbPool};
use crate::model::staff_model::StaffDetail;
use crate::model::store_assignment_model::StoreAssignment;

pub fn get_store_staffs(conn:&mut DbConnect, selected_id:i32) -> Result<Vec<StaffDetail>, String> {
    StoreAssignment::get_store_assignment(conn, selected_id)
}

#[command]
pub fn assign_staff_to_store(state:State<DbPool>, staff_id:i32, store_id:i32) -> Result<(), String> {
    let conn = &mut get_conn(&state)?;
    StoreAssignment::assign_store_staff(conn, staff_id, store_id)
}

pub fn check_store_staff_to_open(conn: &mut DbConnect, selected_id:i32) -> Result<bool, String> {
    StoreAssignment::check_store_staffing(conn, selected_id)
}

pub fn reassign_staff_to_store(conn: &mut DbConnect, staff_id:i32, store_id:i32) -> Result<i32, String> {
    StoreAssignment::reassign_store_staff(conn, staff_id, store_id)
}

pub fn find_staff_store(conn: &mut DbConnect, selected_id:i32) -> Result<i32, String> {
    StoreAssignment::get_staff_store(conn, selected_id)
}

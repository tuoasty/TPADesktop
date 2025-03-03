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

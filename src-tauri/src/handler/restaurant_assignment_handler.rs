use tauri::{command, State};
use crate::{get_conn, DbConnect, DbPool};
use crate::model::restaurant_assignment_model::RestaurantAssignment;
use crate::model::staff_model::{StaffDetail};

#[command]
pub fn get_restaurant_staffs(conn:&mut DbConnect, selected_id:i32) -> Result<Vec<StaffDetail>, String> {
    RestaurantAssignment::get_restaurant_assignments(conn, selected_id)
}

#[command]
pub fn assign_staff_to_restaurant(state:State<DbPool>, staff_id:i32, restaurant_id:i32) -> Result<(), String> {
    let conn = &mut get_conn(&state)?;
    RestaurantAssignment::assign_staff(conn, staff_id, restaurant_id)
}
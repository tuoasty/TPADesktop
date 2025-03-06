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
    RestaurantAssignment::assign_restaurant_staff(conn, staff_id, restaurant_id)
}

pub fn reassign_staff_to_restaurant(conn: &mut DbConnect, staff_id:i32, restaurant_id:i32) -> Result<i32, String> {
    RestaurantAssignment::reassign_restaurant_staff(conn, staff_id, restaurant_id)
}

pub fn check_restaurant_staff_to_open(conn: &mut DbConnect, selected_id:i32) -> Result<bool, String> {
    RestaurantAssignment::check_restaurant_staffing(conn, selected_id)
}

pub fn find_staff_restaurant(conn: &mut DbConnect, selected_id:i32) -> Result<i32, String> {
    RestaurantAssignment::get_staff_restaurant(conn, selected_id)
}
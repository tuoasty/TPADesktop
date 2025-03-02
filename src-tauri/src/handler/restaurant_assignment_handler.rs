use tauri::{command};
use crate::{DbConnect};
use crate::model::restaurant_assignment_model::RestaurantAssignment;
use crate::model::staff_model::{StaffDetail};

#[command]
pub fn get_restaurant_staffs(conn:&mut DbConnect, selected_id:i32) -> Result<Vec<StaffDetail>, String> {
    RestaurantAssignment::get_restaurant_assignments(conn, selected_id)
}
use crate::DbConnect;
use crate::model::staff_model::StaffDetail;
use crate::model::store_assignment_model::StoreAssignment;

pub fn get_store_staffs(conn:&mut DbConnect, selected_id:i32) -> Result<Vec<StaffDetail>, String> {
    StoreAssignment::get_store_assignment(conn, selected_id)
}

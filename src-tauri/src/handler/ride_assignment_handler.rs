use crate::DbConnect;
use crate::model::ride_assignment_model::RideAssignment;
use crate::model::staff_model::StaffDetail;

pub fn get_ride_staffs(conn: &mut DbConnect, selected_id:i32) -> Result<Vec<StaffDetail>, String> {
    RideAssignment::get_ride_assignment(conn, selected_id)
}
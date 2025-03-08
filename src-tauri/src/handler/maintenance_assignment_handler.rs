use chrono::Local;
use crate::DbConnect;
use crate::model::maintenance_assignment_model::{MaintenanceAssignment, NewMaintenanceAssignment};
use crate::model::staff_model::Staff;

pub fn check_maintenance_staff_availability(conn: &mut DbConnect, selected_id:i32) -> Result<bool, String> {

    let staff_assignments = MaintenanceAssignment::get_current_assignments(conn)?;

    let staff_is_busy = staff_assignments
        .iter()
        .any(|assignment| assignment.staff_id == selected_id);

    Ok(!staff_is_busy)
}

pub fn create_maintenance_assignment(conn: &mut DbConnect, id_staff:i32, id_report:i32) -> Result<(), String> {
    let new_assignment = NewMaintenanceAssignment {
        staff_id: id_staff,
        maintenance_report_id: id_report,
        assignment_date: Local::now().naive_local(),
        status: "In Progress".to_string()
    };

    MaintenanceAssignment::create_maintenance_assignment(conn, new_assignment)
}

pub fn find_staff_maintenance(conn: &mut DbConnect, selected_id:i32) -> Result<i32, String> {
    MaintenanceAssignment::get_staff_maintenance(conn, selected_id)
}

pub fn find_maintenance_staff(conn: &mut DbConnect, selected_id:i32) -> Result<Staff, String> {
    MaintenanceAssignment::get_maintenance_staff(conn, selected_id)
}
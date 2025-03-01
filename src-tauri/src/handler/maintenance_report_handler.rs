use tauri::{command, State};
use crate::{get_conn, DbPool};
use crate::handler::staff_handler::{find_staff_per_role};
use crate::model::maintenance_report_model::{MaintenanceReport, MaintenanceReportDetail};
use crate::model::staff_model::StaffDetail;

#[command]
pub fn find_all_maintenance_report(state:State<DbPool>) -> Result<Vec<MaintenanceReportDetail>, String> {
    let conn = &mut get_conn(&state)?;

    MaintenanceReport::get_all_maintenance_report(conn)
}

#[command]
pub fn find_all_maintenance_staff(state:State<DbPool>) -> Result<Vec<StaffDetail>, String> {
    let conn = &mut get_conn(&state)?;

    find_staff_per_role(conn, "Maintenance Staff".to_string())
}

#[command]
pub fn accept_request(state:State<DbPool>, selected_staff_id:i32, selected_maintenance_id:i32) -> Result<(), String> {
    let conn = &mut get_conn(&state)?;

    let staff_available = MaintenanceReport::check_staff_availability(conn, selected_staff_id)?;

    if !staff_available {
        return Err("Staff Unavailable".to_string())
    };

    MaintenanceReport::assign_staff_to_report(conn, selected_staff_id, selected_maintenance_id)
}

#[command]
pub fn reject_request(state:State<DbPool>, selected_maintenance_id:i32) -> Result<(), String> {
    let conn = &mut get_conn(&state)?;

    MaintenanceReport::reject_request(conn, selected_maintenance_id)
}
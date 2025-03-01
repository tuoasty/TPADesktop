use tauri::{command, State};
use crate::{get_conn, DbPool};
use crate::handler::staff_handler::{find_staff_per_role};
use crate::models::{MaintenanceReport, MaintenanceReportDetail, Staff, StaffDetail};

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
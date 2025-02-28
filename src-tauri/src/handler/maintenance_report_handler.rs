use tauri::{command, State};
use crate::{get_conn, DbPool};
use crate::models::{MaintenanceReport, MaintenanceReportDetail};

#[command]
pub fn find_all_maintenance_report(state:State<DbPool>) -> Result<Vec<MaintenanceReportDetail>, String> {
    let conn = &mut get_conn(&state)?;

    MaintenanceReport::get_all_maintenance_report(conn)
}
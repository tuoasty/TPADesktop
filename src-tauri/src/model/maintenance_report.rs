use crate::DbConnect;
use crate::models::{MaintenanceReport, MaintenanceReportDetail};
use crate::schema::maintenance_reports::dsl::maintenance_reports;
use diesel::prelude::*;
use crate::handler::ride_handler::find_ride;

impl MaintenanceReport {
    pub fn get_all_maintenance_report(conn:&mut DbConnect) -> Result<Vec<MaintenanceReportDetail>, String> {
        let reports = maintenance_reports.select(MaintenanceReport::as_select()).load(conn).map_err(|e| e.to_string())?;

        let report_details: Vec<MaintenanceReportDetail> = reports
            .into_iter()
            .map(|report| {

                let ride = find_ride(conn, report.ride_id).unwrap();

                MaintenanceReportDetail {
                    id:report.id,
                    ride_id:report.ride_id,
                    ride_name:ride.name,
                    staff_id:None,
                    staff_name:None,
                    description:report.description,
                    status:report.status
                }
            })
            .collect();

        Ok(report_details)
    }
}
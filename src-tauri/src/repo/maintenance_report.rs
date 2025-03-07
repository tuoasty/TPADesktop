use crate::DbConnect;
use crate::model::maintenance_report_model::{MaintenanceReport, MaintenanceReportDetail, NewMaintenanceReport};
use crate::schema::maintenance_reports::dsl::maintenance_reports;
use diesel::prelude::*;
use crate::handler::maintenance_assignment_handler::{check_maintenance_staff_availability, create_maintenance_assignment, find_staff_maintenance};
use crate::handler::ride_handler::{accept_ride_maintenance, find_ride, reject_ride_maintenance};
use crate::schema::maintenance_reports::{id, ride_id, status};
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

    pub fn create_maintenance_report(conn:&mut DbConnect, new_report:NewMaintenanceReport) -> Result<(), String> {
        diesel::insert_into(maintenance_reports)
            .values(new_report)
            .execute(conn)
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn check_staff_availability(conn: &mut DbConnect, selected_staff_id:i32) -> Result<bool, String> {
        check_maintenance_staff_availability(conn, selected_staff_id)
    }

    pub fn assign_staff_to_report(conn:&mut DbConnect, selected_staff_id:i32, selected_maintenance_id:i32) -> Result<(), String> {
        create_maintenance_assignment(conn, selected_staff_id, selected_maintenance_id)?;

        let maintenance_ride_id = MaintenanceReport::get_maintenance_ride_id(conn, selected_maintenance_id)?;

        accept_ride_maintenance(conn, maintenance_ride_id)?;

        Self::set_maintenance_status(conn, selected_maintenance_id, "In Progress".to_string())
    }

    pub fn set_maintenance_status(conn:&mut DbConnect, selected_maintenance_id:i32, new_status:String) -> Result<(), String> {
        diesel::update(maintenance_reports.filter(id.eq(selected_maintenance_id)))
            .set(status.eq(new_status))
            .execute(conn)
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn reject_request(conn:&mut DbConnect, selected_maintenance_id:i32) -> Result<(), String> {
        let selected_ride_id = MaintenanceReport::get_maintenance_ride_id(conn, selected_maintenance_id)?;

        Self::set_maintenance_status(conn, selected_maintenance_id, "Rejected".to_string())?;
        reject_ride_maintenance(conn, selected_ride_id)?;
        Ok(())
    }

    pub fn get_maintenance_ride_id(conn:&mut DbConnect, selected_maintenance_id:i32) -> Result<i32, String> {
        maintenance_reports
            .filter(id.eq(&selected_maintenance_id))
            .select(ride_id)
            .first::<i32>(conn)
            .map_err(|e| e.to_string())
    }

    pub fn get_staff_maintenance(conn: &mut DbConnect, selected_id:i32) -> Result<Self, String> {
        let staff_maintenance_id = find_staff_maintenance(conn, selected_id)?;

        maintenance_reports.filter(id.eq(&staff_maintenance_id))
            .select(MaintenanceReport::as_select())
            .first(conn)
            .map_err(|e| e.to_string())
    }
}
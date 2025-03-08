use crate::schema::maintenance_assignments;
use crate::schema::maintenance_assignments::dsl::*;
use crate::DbConnect;
use diesel::prelude::*;
use crate::handler::staff_handler::find_staff;
use crate::model::maintenance_assignment_model::{MaintenanceAssignment, NewMaintenanceAssignment};
use crate::model::staff_model::Staff;
use crate::schema::maintenance_assignments::status;

impl MaintenanceAssignment {
    pub fn get_current_assignments(conn: &mut DbConnect) -> Result<Vec<Self>, String> {
        maintenance_assignments
            .filter(status.eq("In Progress"))
            .load(conn)
            .map_err(|e| e.to_string())
    }

    pub fn create_maintenance_assignment(conn: &mut DbConnect, new_assignment:NewMaintenanceAssignment) -> Result<(), String> {
        diesel::insert_into(maintenance_assignments::table)
            .values(&new_assignment)
            .execute(conn)
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn get_staff_maintenance(conn: &mut DbConnect, selected_id:i32) -> Result<i32, String> {
        maintenance_assignments
            .filter(staff_id.eq(&selected_id))
            .select(maintenance_report_id)
            .first(conn)
            .map_err(|e| e.to_string())
    }

    pub fn get_maintenance_staff(conn: &mut DbConnect, selected_id:i32) -> Result<Staff, String> {
        let maintenance_staff_id = maintenance_assignments.filter(maintenance_report_id.eq(&selected_id))
            .select(staff_id)
            .first(conn)
            .map_err(|e| e.to_string())?;

        find_staff(conn, maintenance_staff_id)
    }
}

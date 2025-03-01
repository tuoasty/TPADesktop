use crate::schema::maintenance_assignments;
use crate::schema::maintenance_assignments::dsl::*;
use crate::DbConnect;
use diesel::prelude::*;
use crate::model::maintenance_assignment_model::{MaintenanceAssignment, NewMaintenanceAssignment};
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
}

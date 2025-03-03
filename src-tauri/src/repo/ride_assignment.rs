use crate::DbConnect;
use crate::model::ride_assignment_model::RideAssignment;
use crate::model::staff_model::StaffDetail;
use crate::schema::ride_assignments::dsl::ride_assignments;
use crate::schema::ride_assignments::{ride_id, role};
use crate::schema::staffs::dsl::staffs;
use crate::schema::staffs::name;
use diesel::prelude::*;

impl RideAssignment {
    pub fn get_ride_assignment(conn:&mut DbConnect, selected_id:i32) -> Result<Vec<StaffDetail>, String> {
        let res: Vec<(i32, String, String)> = ride_assignments
            .filter(ride_id.eq(selected_id))
            .inner_join(staffs)
            .select((
                crate::schema::staffs::id,
                name,
                role,
            ))
            .load(conn)
            .map_err(|e| e.to_string())?;

        let staff_details = res
            .into_iter()
            .map(|(new_id, new_name, new_role)| StaffDetail {
                id:new_id,
                name:new_name,
                role:new_role
            })
            .collect();

        Ok(staff_details)
    }
}
use crate::DbConnect;
use crate::model::staff_model::StaffDetail;
use crate::model::store_assignment_model::StoreAssignment;
use crate::schema::staffs::dsl::staffs;
use crate::schema::staffs::name;
use crate::schema::store_assignments::dsl::store_assignments;
use diesel::prelude::*;
use crate::schema::store_assignments::{role, store_id};

impl StoreAssignment {
    pub fn get_store_assignment(conn:&mut DbConnect, selected_id:i32) -> Result<Vec<StaffDetail>, String> {
        let res: Vec<(i32, String, String)> = store_assignments
            .filter(store_id.eq(selected_id))
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
use crate::DbConnect;
use crate::model::staff_model::StaffDetail;
use crate::model::store_assignment_model::{NewStoreAssignment, StoreAssignment};
use crate::schema::staffs::dsl::staffs;
use crate::schema::staffs::name;
use crate::schema::store_assignments::dsl::store_assignments;
use diesel::prelude::*;
use crate::schema::store_assignments::{role, staff_id, store_id};

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

    pub fn assign_store_staff(conn: &mut DbConnect, new_staff_id:i32, new_store_id:i32) -> Result<(), String> {
        let existing_assignment =
            store_assignments.filter(staff_id.eq(new_staff_id))
                .select(store_id)
                .load::<i32>(conn)
                .map_err(|e| e.to_string())?;

        if existing_assignment.contains(&new_store_id) {
            return Err("Staff is already assigned to this store".to_string());
        }

        if !existing_assignment.is_empty(){
            return Err("Staff is assigned to another store".to_string());
        };

        diesel::insert_into(store_assignments)
            .values(
                NewStoreAssignment{
                    staff_id:new_staff_id,
                    store_id:new_store_id,
                    role:"Sales Associate".to_string(),
                }
            )
            .execute(conn)
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
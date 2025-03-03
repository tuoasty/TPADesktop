use crate::DbConnect;
use crate::model::staff_model::StaffDetail;
use crate::model::store_assignment_model::{NewStoreAssignment, StoreAssignment};
use crate::schema::staffs::dsl::staffs;
use crate::schema::staffs::name;
use crate::schema::store_assignments::dsl::store_assignments;
use diesel::prelude::*;
use crate::schema::store_assignments::{id, role, staff_id, store_id};

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
            return Err("STAFF ASSIGNED".to_string());
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

    pub fn check_store_staffing(conn: &mut DbConnect, selected_id:i32) -> Result<bool, String> {
        let staff_assignments:Vec<i32> = store_assignments.filter(store_id.eq(&selected_id))
            .select(id)
            .load(conn)
            .map_err(|e| e.to_string())?;

        if staff_assignments.len() >= 2 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn reassign_store_staff(conn:&mut DbConnect, new_staff_id:i32, new_store_id:i32) -> Result<i32, String> {
        let deleted_store_id:i32 = diesel::delete(store_assignments
            .filter(staff_id.eq(&new_staff_id)))
            .returning(store_id)
            .get_result(conn)
            .map_err(|e| e.to_string())?;

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

        Ok(deleted_store_id)
    }
}
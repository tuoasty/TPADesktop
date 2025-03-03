use crate::DbConnect;
use crate::model::ride_assignment_model::{NewRideAssignment, RideAssignment};
use crate::model::staff_model::StaffDetail;
use crate::schema::ride_assignments::dsl::ride_assignments;
use crate::schema::ride_assignments::{id, ride_id, role, staff_id};
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

    pub fn assign_ride_staff(conn: &mut DbConnect, new_staff_id:i32, new_ride_id:i32) -> Result<(), String> {
        let existing_assignment =
            ride_assignments.filter(staff_id.eq(new_staff_id))
                .select(ride_id)
                .load::<i32>(conn)
                .map_err(|e| e.to_string())?;

        if existing_assignment.contains(&new_ride_id) {
            return Err("Staff is already assigned to this ride".to_string());
        }

        if !existing_assignment.is_empty(){
            return Err("STAFF ASSIGNED".to_string());
        };

        diesel::insert_into(ride_assignments)
            .values(
                NewRideAssignment{
                    staff_id:new_staff_id,
                    ride_id:new_ride_id,
                    role:"Ride Staff".to_string(),
                }
            )
            .execute(conn)
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn reassign_ride_staff(conn: &mut DbConnect, new_staff_id:i32, new_ride_id:i32) -> Result<(), String> {
        diesel::delete(ride_assignments
            .filter(staff_id.eq(&new_staff_id)))
            .execute(conn)
            .map_err(|e| e.to_string())?;

        diesel::insert_into(ride_assignments)
            .values(
                NewRideAssignment{
                    staff_id:new_staff_id,
                    ride_id:new_ride_id,
                    role:"Ride Staff".to_string(),
                }
            )
            .execute(conn)
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
use crate::DbConnect;
use crate::model::restaurant_assignment_model::{NewRestaurantAssignment, RestaurantAssignment};
use crate::schema::restaurant_assignments::dsl::*;
use diesel::prelude::*;
use crate::handler::staff_handler::find_staff_role;
use crate::model::staff_model::{StaffDetail};
use crate::schema::staffs::dsl::staffs;
use crate::schema::staffs::name;

impl RestaurantAssignment {
    pub fn get_restaurant_assignments(conn:&mut DbConnect, selected_id:i32) -> Result<Vec<StaffDetail>, String> {
        let res: Vec<(i32, String, String)> = restaurant_assignments
            .filter(restaurant_id.eq(selected_id))
            .inner_join(staffs)
            .select((
                crate::schema::staffs::id,
                name,
                role
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

    pub fn assign_restaurant_staff(conn: &mut DbConnect, new_staff_id:i32, new_restaurant_id:i32) -> Result<(), String> {
        let existing_assignment =
            restaurant_assignments.filter(staff_id.eq(new_staff_id))
                .select(restaurant_id)
                .load::<i32>(conn)
                .map_err(|e| e.to_string())?;

        if existing_assignment.contains(&new_restaurant_id) {
            return Err("Staff is already assigned to this restaurant".to_string());
        }

        if !existing_assignment.is_empty(){
            return Err("STAFF ASSIGNED".to_string());
        };

        let staff_role = find_staff_role(conn, new_staff_id)?;

        diesel::insert_into(restaurant_assignments)
            .values(
                NewRestaurantAssignment{
                    staff_id:new_staff_id,
                    restaurant_id:new_restaurant_id,
                    role:staff_role
                }
            )
            .execute(conn)
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn reassign_restaurant_staff(conn: &mut DbConnect, new_staff_id:i32, new_restaurant_id:i32) -> Result<i32, String> {
        let deleted_ride_id = diesel::delete(restaurant_assignments
            .filter(staff_id.eq(&new_staff_id)))
            .returning(restaurant_id)
            .get_result(conn)
            .map_err(|e| e.to_string())?;

        let staff_role = find_staff_role(conn, new_staff_id)?;

        diesel::insert_into(restaurant_assignments)
            .values(
                NewRestaurantAssignment{
                    staff_id:new_staff_id,
                    restaurant_id:new_restaurant_id,
                    role:staff_role,
                }
            )
            .execute(conn)
            .map_err(|e| e.to_string())?;

        Ok(deleted_ride_id)
    }
    pub fn check_restaurant_staffing(conn: &mut DbConnect, selected_id:i32) -> Result<bool, String> {
        let waiters:Vec<i32> = restaurant_assignments.filter(restaurant_id.eq(&selected_id))
            .filter(role.eq("Waiter".to_string()))
            .select(id)
            .load(conn)
            .map_err(|e| e.to_string())?;

        let chefs:Vec<i32> = restaurant_assignments.filter(restaurant_id.eq(&selected_id))
            .filter(role.eq("Chef".to_string()))
            .select(id)
            .load(conn)
            .map_err(|e| e.to_string())?;

        if waiters.len() >= 2 && chefs.len() >= 2 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn get_staff_restaurant(conn: &mut DbConnect, selected_id:i32) -> Result<i32, String> {
        restaurant_assignments.filter(staff_id.eq(&selected_id))
            .select(restaurant_id)
            .first(conn)
            .map_err(|e| e.to_string())
    }
}
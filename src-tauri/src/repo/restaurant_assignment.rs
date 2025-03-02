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

    pub fn assign_staff(conn: &mut DbConnect, new_staff_id:i32, new_restaurant_id:i32) -> Result<(), String> {
        let existing_assignment =
            restaurant_assignments.filter(staff_id.eq(new_staff_id))
                .select(id)
                .load::<i32>(conn)
                .map_err(|e| e.to_string())?;

        if !existing_assignment.is_empty(){
            return Err("Staff is assigned to another restaurant".to_string());
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
}
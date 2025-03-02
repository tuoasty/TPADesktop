use crate::DbConnect;
use crate::model::restaurant_assignment_model::RestaurantAssignment;
use crate::schema::restaurant_assignments::dsl::*;
use diesel::prelude::*;
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
}
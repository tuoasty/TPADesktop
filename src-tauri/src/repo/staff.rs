use crate::model::staff_model::{NewStaff, Staff, StaffDetail};
use crate::schema::staffs::dsl::staffs;
use crate::schema::staffs::{name, role};
use crate::DbConnect;
use diesel::prelude::*;
impl Staff {
    pub fn get_staff(conn: &mut DbConnect, username: &str) -> Result<Self, String> {
        staffs
            .filter(name.eq(&username))
            .first(conn)
            .map_err(|e| e.to_string())
    }

    pub fn create_staff(conn: &mut DbConnect, new_staff:NewStaff) -> Result<String, String> {
        diesel::insert_into(staffs)
            .values(&new_staff)
            .execute(conn)
            .map_err(|e| e.to_string())?;

        Ok("Successfully created staff account!".to_string())
    }

    pub fn get_staff_per_role(conn: &mut DbConnect, staff_role:String) -> Result<Vec<StaffDetail>, String> {
        let staff_list = staffs
            .filter(role.eq(&staff_role))
            .select(Staff::as_select())
            .load(conn).unwrap();

        let staff_details: Vec<StaffDetail> = staff_list
            .into_iter()
            .map(|staff| {
                StaffDetail {
                    id: staff.id,
                    name: staff.name,
                    role: staff.role
                }
            })
            .collect();

        Ok(staff_details)
    }
}

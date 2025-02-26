use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use tauri::{command, State};
use crate::{get_conn, CurrentStaff, DbPool};
use crate::models::{NewStaff, Staff};

#[command]
pub fn create_staff(state: State<DbPool>, name:String, password:String, role:String) -> Result<String, String>{
   use crate::schema::staffs::dsl::staffs;
   let conn = &mut get_conn(&state)?;
   let hashed_password = hash(password, DEFAULT_COST).map_err(|_| "failed to hash".to_string())?;

   let staff = NewStaff {
      name,
      password: hashed_password,
      role,
   };

   diesel::insert_into(staffs)
       .values(&staff)
       .execute(conn)
       .map_err(|e| e.to_string())?;

   Ok("Successfully registered".to_string())
}

#[command]
pub fn get_current_staff(current_staff: State<CurrentStaff>) -> Result<Option<(i32, String, String)>, String>{
   let staff = current_staff.0.lock().expect("Current staff access error").clone();
   Ok(staff)
}

#[command]
pub fn login_staff(state: State<DbPool>, current_staff:State<CurrentStaff>, name:String, password:String) -> Result<String, String>{
   let conn = &mut get_conn(&state)?;

   let staff = Staff::get_staff(conn, &name)?;

   if verify(password, &staff.password).map_err(|_| "Error verifying password".to_string())?{
      *current_staff.0.lock().unwrap() = Some((staff.id, staff.name.clone(), staff.role.clone()));
      Ok(format!("Welcome {}", &staff.name))
   } else {
      Err("Incorrect credentials".to_string())
   }
}

#[command]
pub fn logout_staff(current_staff: State<CurrentStaff>) -> Result<String, String> {
   let mut lock = current_staff.0.lock().unwrap();
   *lock = None;
   Ok("Successfully logout".to_string())
}



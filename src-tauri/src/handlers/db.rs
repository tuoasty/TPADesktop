use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;
use tauri::{command, State};
use crate::{get_conn, DbPool};
use crate::models::{NewStaff};
#[command]
pub fn register_staff(state: State<DbPool>, username:String, input_password:String, user_role:String) -> Result<String, String>{
   use crate::schema::staffs::dsl::*;
   let conn = &mut get_conn(&state)?;
   let hashed_password = hash(input_password, DEFAULT_COST).map_err(|_| "failed to hash".to_string())?;

   let new_staff = NewStaff {
      name: username,
      password: hashed_password,
      role:user_role,
   };

   diesel::insert_into(staffs)
       .values(&new_staff)
       .execute(conn)
       .map_err(|e| e.to_string())?;

   Ok("Successfully registered".to_string())
}


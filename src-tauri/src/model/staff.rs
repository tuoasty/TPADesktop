use bcrypt::verify;
use tauri::{command, State};
use crate::{get_conn, CurrentStaff, DbPool};
use crate::models::Staff;
use crate::schema::staffs::dsl::*;
use diesel::prelude::*;

#[command]
pub fn get_staff(current_staff: State<CurrentStaff>) -> Result<Option<(i32, String, String)>, String>{
    let staff = current_staff.0.lock().expect("Current staff access error").clone();
    Ok(staff)
}

#[command]
pub fn login_staff(state: State<DbPool>, current_staff:State<CurrentStaff>, username:String, input_password:String) -> Result<String, String>{
    let conn = &mut get_conn(&state)?;

    let staff:Staff = staffs
        .filter(name.eq(&username))
        .first(conn)
        .map_err(|e| e.to_string())?;

    if verify(input_password, &staff.password).map_err(|_| "Error verifying password".to_string())?{
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


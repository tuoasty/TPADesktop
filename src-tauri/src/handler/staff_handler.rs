use bcrypt::{hash, verify, DEFAULT_COST};
use tauri::{command, State};
use crate::{get_conn, CurrentStaff, DbConnect, DbPool};
use crate::model::staff_model::{NewStaff, Staff, StaffDetail};

#[command]
pub fn create_staff(state: State<DbPool>, name:String, password:String, role:String) -> Result<String, String>{
    let conn = &mut get_conn(&state)?;
    let hashed_password = hash(password, DEFAULT_COST).map_err(|_| "failed to hash".to_string())?;

    let staff = NewStaff {
        name,
        password: hashed_password,
        role,
    };

    Staff::create_staff(conn, staff)
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
pub async fn verify_authentication(current_staff:State<'_,CurrentStaff>, allowed_roles:Vec<String>) -> Result<bool, String> {
    let current_staff = current_staff.0.lock().unwrap();

    if current_staff.is_none(){
        return Ok(false);
    };

    if allowed_roles.is_empty(){
        return Ok(true);
    };


    let(_,_, role) = current_staff.as_ref().unwrap();

    let is_allowed = allowed_roles.contains(&role);

    Ok(is_allowed)
}

#[command]
pub async fn verify_login(current_staff:State<'_,CurrentStaff>) -> Result<bool, String>{
    let current_staff = !current_staff.0.lock().unwrap().is_none();
    Ok(current_staff)
}

#[command]
pub fn logout_staff(current_staff: State<CurrentStaff>) -> Result<String, String> {
    let mut lock = current_staff.0.lock().unwrap();
    *lock = None;
    Ok("Successfully logout".to_string())
}

pub fn find_staff_per_role(conn: &mut DbConnect, staff_role:String) -> Result<Vec<StaffDetail>, String> {
    Staff::get_staff_per_role(conn, staff_role)
}
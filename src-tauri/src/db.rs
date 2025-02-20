use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, PooledConnection};
use r2d2::Pool;
use tauri::{command, State};
use crate::CurrentUser;
use crate::models::{NewUser, User};
use crate::schema::users::dsl::*;

type DbPool = Pool<ConnectionManager<PgConnection>>;

fn get_conn(pool:&DbPool) -> Result<PooledConnection<ConnectionManager<PgConnection>>, String> {
   pool.get().map_err(|_| "failed to get DB pool".to_string())
}

#[command]
pub fn register_user(state: State<DbPool>, username:String, input_password:String, role:bool) -> Result<String, String>{
   use crate::schema::users::dsl::*;
   let conn = &mut get_conn(&state)?;
   let hashed_password = hash(input_password, DEFAULT_COST).map_err(|_| "failed to hash".to_string())?;

   let new_user = NewUser {
      name: username,
      password: hashed_password,
      admin:role,
   };

   diesel::insert_into(users)
       .values(&new_user)
       .execute(conn)
       .map_err(|e| e.to_string())?;

   Ok("Successfully registered".to_string())
}

#[command]
pub fn get_user(current_user: State<CurrentUser>) -> Result<Option<(i32, String)>, String>{
   let user = current_user.0.lock().expect("Current user access error").clone();
   Ok(user)
}

#[command]
pub fn login_user(state: State<DbPool>, current_user:State<CurrentUser>, username:String, input_password:String) -> Result<String, String>{
   let conn = &mut get_conn(&state)?;

   let user:User = users
       .filter(name.eq(&username))
       .first(conn)
       .map_err(|e| e.to_string())?;

   if verify(input_password, &user.password).map_err(|_| "Error verifying password".to_string())?{
      *current_user.0.lock().unwrap() = Some((user.id, user.name.clone()));
      Ok(format!("Welcome {}", &user.name))
   } else {
      Err("Incorrect credentials".to_string())
   }
}

#[command]
pub fn logout_user(current_user: State<CurrentUser>) -> Result<String, String> {
   let mut lock = current_user.0.lock().unwrap();
   *lock = None;
   Ok("Successfully logout".to_string())
}


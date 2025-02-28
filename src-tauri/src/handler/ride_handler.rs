use diesel::{ExpressionMethods, RunQueryDsl};
use tauri::{command, State};
use crate::{get_conn, DbPool};
use crate::models::{Ride, RideDetail};
use crate::schema::rides::dsl::rides;
use crate::schema::rides::{id, status};

#[command]
pub fn find_all_ride(state:State<DbPool>) -> Result<Vec<RideDetail>, String> {
    let conn = &mut get_conn(&state)?;
    let ride_details:Vec<RideDetail> = Ride::get_all_ride(conn)?;

    Ok(ride_details)
}

#[command]
pub fn change_ride_status(state:State<DbPool>, ride_id:i32, ride_status:String) -> Result<(), String> {
    let conn = &mut get_conn(&state)?;

    if ride_status == "Open" {
        diesel::update(rides).filter(id.eq(ride_id)).set(status.eq("Closed")).execute(conn).unwrap();
    } else if ride_status == "Closed" {
        diesel::update(rides).filter(id.eq(ride_id)).set(status.eq("Open")).execute(conn).unwrap();
    };

    Ok(())
}
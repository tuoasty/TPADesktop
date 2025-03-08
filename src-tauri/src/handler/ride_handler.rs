use chrono::{Local, NaiveTime};
use tauri::{command, State};
use crate::{get_conn, DbConnect, DbPool, RedisCache};
use crate::handler::image_handler::get_image_data;
use crate::handler::ride_assignment_handler::get_ride_staffs;
use crate::model::ride_model::{NewRide, Ride, RideDetail};
use crate::model::ride_proposal_model::RideProposal;

#[command]
pub fn find_all_ride(state:State<DbPool>) -> Result<Vec<RideDetail>, String> {
    let conn = &mut get_conn(&state)?;
    let ride_details:Vec<RideDetail> = Ride::get_all_ride(conn)?;

    Ok(ride_details)
}

#[command]
pub fn find_ride_by_id(state: State<DbPool>, redis_state: State<RedisCache>, selected_id: i32) -> Result<RideDetail, String> {
    if let Some(cached_ride) = redis_state.get_ride(selected_id) {
        return Ok(cached_ride);
    }

    let conn = &mut get_conn(&state)?;
    let ride = Ride::get_ride(conn, selected_id)?;

    let ride_detail = RideDetail {
        id: ride.id,
        name: ride.name,
        open_time: ride.open_time.to_string(),
        close_time: ride.close_time.to_string(),
        image_data: get_image_data(conn, ride.image_id)?,
        price: ride.price,
        status: {
            let current_time = Local::now().time();

            if ride.status == "Shut Down"  || ride.status == "Maintenance in Progress" || ride.status == "Pending Maintenance" {
                ride.status
            } else if current_time < ride.open_time || current_time > ride.close_time {
                "Closed for the day".to_string()
            } else {
                ride.status
            }
        },
        staffs: get_ride_staffs(conn, ride.id)?
    };

    let _ = redis_state.set_ride(&ride_detail);

    Ok(ride_detail)
}

pub fn find_ride_price(conn: &mut DbConnect, selected_id:i32) -> Result<i32, String> {
    Ride::get_ride_price(conn, selected_id)
}
#[command]
pub fn change_ride_status(state:State<DbPool>, ride_id:i32, ride_status:String) -> Result<(), String> {
    let conn = &mut get_conn(&state)?;

    let mut new_status = ride_status.clone();

    if ride_status == "Open" {
        new_status = "Closed".to_string();
    } else if ride_status == "Closed" {
        new_status = "Open".to_string();
    }

    Ride::update_ride_status(conn, ride_id, new_status)
}

#[command]
pub fn report_ride_maintenance(state:State<DbPool>, ride_id:i32, description:String) -> Result<(), String> {
    let conn = &mut get_conn(&state)?;

    Ride::report_ride_maintenance(conn, ride_id, description)
}
#[command]
pub fn reassign_ride_and_check_status(state:State<DbPool>, new_staff_id:i32, new_ride_id:i32) -> Result<(), String> {
    let conn = &mut get_conn(&state)?;

    let removed_ride_staff_id = Ride::reassign_ride_staff(conn, new_staff_id, new_ride_id)?;

    Ride::check_ride_assignment_and_update(conn, removed_ride_staff_id)
}

pub fn accept_ride_maintenance(conn: &mut DbConnect, ride_id:i32) -> Result<(), String> {
    Ride::update_ride_status(conn, ride_id, "Maintenance in Progress".to_string())
}

pub fn reject_ride_maintenance(conn: &mut DbConnect, ride_id:i32) -> Result<(), String> {
    Ride::update_ride_status(conn, ride_id, "Closed".to_string())
}

pub fn find_ride(conn:&mut DbConnect, ride_id:i32) -> Result<Ride, String> {
    Ride::get_ride(conn, ride_id)
}

#[command]
pub fn find_staff_ride(state:State<DbPool>, selected_id:i32) -> Result<RideDetail, String> {
    let conn = &mut get_conn(&state)?;
    Ride::get_staff_ride(conn, selected_id)
}

pub fn create_new_ride(conn: &mut DbConnect, proposal:RideProposal) -> Result<(), String> {
    let new_ride = NewRide {
        name:proposal.name,
        image_id:proposal.image_id.unwrap(),
        open_time:NaiveTime::from_hms_opt(7,0,0).unwrap(),
        close_time:NaiveTime::from_hms_opt(19,0,0).unwrap(),
        price:proposal.price,
        status:"In Construction".to_string()
    };

    Ride::create_ride(conn, new_ride)
}

pub fn close_ride(conn:&mut DbConnect, ride_id:i32) -> Result<(), String> {
    Ride::close_ride(conn, ride_id)
}
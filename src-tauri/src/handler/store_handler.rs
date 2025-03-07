use chrono::{Local, NaiveTime};
use tauri::{command, State};
use crate::{get_conn, DbConnect, DbPool};
use crate::handler::image_handler::get_image_data;
use crate::handler::souvenir_handler::find_store_souvenir;
use crate::handler::store_assignment_handler::get_store_staffs;
use crate::model::store_model::{NewStore, Store, StoreDetail};
use crate::model::store_proposal_model::StoreProposal;

#[command]
pub fn find_all_store(state: State<DbPool>) -> Result<Vec<StoreDetail>, String> {
    let conn = &mut get_conn(&state)?;

    let store_details: Vec<StoreDetail> = Store::get_all_stores(conn)?;

    Ok(store_details)
}

#[command]
pub fn change_store_status(state:State<DbPool>, store_id:i32, store_status:String) -> Result<(), String> {
    let conn = &mut get_conn(&state)?;

    let mut new_status = store_status.clone();

    if store_status == "Open" {
        new_status = "Closed".to_string();
    } else if store_status == "Closed" {
        new_status = "Open".to_string();
    }

    Store::update_store_status(conn, store_id, new_status)
}

#[command]
pub fn reassign_store_and_check_status(state:State<DbPool>, new_staff_id:i32, new_store_id:i32) -> Result<(), String> {
    let conn = &mut get_conn(&state)?;

    let deleted_store_id = Store::reassign_store_staff(conn, new_staff_id, new_store_id)?;

    Store::check_store_assignment_and_update(conn, deleted_store_id)
}

#[command]
pub fn find_store_by_id(state:State<DbPool>, selected_id:i32) -> Result<StoreDetail, String> {
    let conn = &mut get_conn(&state)?;
    let store = Store::get_store(conn, selected_id)?;

    let store_detail = StoreDetail {
        id:store.id,
        name:store.name,
        open_time:store.open_time.to_string(),
        close_time:store.close_time.to_string(),
        image_data:get_image_data(conn, store.image_id)?,
        status: {
            let current_time = Local::now().time();

            if store.status == "Shut Down" {
                store.status
            } else if current_time < store.open_time || current_time > store.close_time {
                "Closed for the day".to_string()
            } else {
                store.status
            }
        },
        souvenirs:find_store_souvenir(conn, store.id)?,
        staffs:get_store_staffs(conn, store.id)?
    };

    Ok(store_detail)
}

#[command]
pub fn find_staff_store(state: State<DbPool>, selected_id:i32) -> Result<StoreDetail, String> {
    let conn = &mut get_conn(&state)?;
    let store = Store::get_staff_store(conn, selected_id)?;
    let image_data = get_image_data(conn, store.image_id)?;
    let souvenirs = find_store_souvenir(conn, store.id)?;
    let staffs = get_store_staffs(conn, store.id)?;

    let store_detail = StoreDetail {
        id:store.id,
        name:store.name,
        open_time:store.open_time.to_string(),
        close_time:store.close_time.to_string(),
        status: {
            let current_time = Local::now().time();

            if store.status == "Shut Down" {
                store.status
            } else if current_time < store.open_time || current_time > store.close_time {
                "Closed for the day".to_string()
            } else {
                store.status
            }
        },
        image_data,
        souvenirs,
        staffs
    };

    Ok(store_detail)
}

pub fn create_new_store(conn: &mut DbConnect, proposal:StoreProposal) -> Result<(), String> {
    let new_store = NewStore {
        name:proposal.name,
        image_id:proposal.image_id.unwrap(),
        open_time:NaiveTime::from_hms_opt(7,0,0).unwrap(),
        close_time:NaiveTime::from_hms_opt(19,0,0).unwrap(),
        status:"In Construction".to_string()
    };

    Store::create_store(conn, new_store)
}

pub fn close_store(conn: &mut DbConnect, store_id:i32) -> Result<(), String> {
    Store::close_store(conn, store_id)
}

pub fn find_store(conn: &mut DbConnect, store_id:i32) -> Result<Store, String> {
    Store::get_store(conn, store_id)
}


use crate::model::restaurant_model::{Restaurant, RestaurantDetail};
use crate::{get_conn, DbPool};
use tauri::{command, State};
use crate::handler::staff_handler::find_staff_per_role;
use crate::model::staff_model::StaffDetail;

#[command]
pub fn find_all_restaurant(state: State<DbPool>) -> Result<Vec<RestaurantDetail>, String> {
    let conn = &mut get_conn(&state)?;

    let restaurant_details: Vec<RestaurantDetail> = Restaurant::get_restaurant_with_menu(conn)?;

    Ok(restaurant_details)
}

#[command]
pub fn find_all_consumption_staff(state: State<DbPool>) -> Result<Vec<StaffDetail>, String> {
    let conn = &mut get_conn(&state)?;

    let waiters = find_staff_per_role(conn, "Waiter".to_string())?;
    let chefs = find_staff_per_role(conn, "Chef".to_string())?;

    let all_staffs:Vec<StaffDetail> = waiters.into_iter().chain(chefs.into_iter()).collect();

    Ok(all_staffs)
}

#[command]
pub fn change_restaurant_status(state:State<DbPool>, restaurant_id:i32, restaurant_status:String) -> Result<(), String> {
    let conn = &mut get_conn(&state)?;

    let mut new_status = restaurant_status.clone();

    if restaurant_status == "Open" {
        new_status = "Closed".to_string();
    } else if restaurant_status == "Closed" {
        new_status = "Open".to_string();
    }

    Restaurant::update_restaurant_status(conn, restaurant_id, new_status)
}


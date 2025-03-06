use chrono::Local;
use crate::model::restaurant_model::{Restaurant, RestaurantDetail};
use crate::{get_conn, DbPool};
use tauri::{command, State};
use crate::handler::image_handler::get_image_data;
use crate::handler::menu_handler::find_restaurant_menu;
use crate::handler::restaurant_assignment_handler::get_restaurant_staffs;
use crate::handler::staff_handler::find_staff_per_role;
use crate::model::staff_model::StaffDetail;
#[command]
pub fn find_all_restaurant(state: State<DbPool>) -> Result<Vec<RestaurantDetail>, String> {
    let conn = &mut get_conn(&state)?;

    let restaurant_details: Vec<RestaurantDetail> = Restaurant::get_restaurant_with_menu(conn)?;

    Ok(restaurant_details)
}

#[command]
pub fn find_restaurant_by_id(state:State<DbPool>, selected_id:i32) -> Result<RestaurantDetail, String> {
    let conn = &mut get_conn(&state)?;
    let restaurant = Restaurant::get_restaurant(conn, selected_id)?;

    let restaurant_detail = RestaurantDetail {
        id:restaurant.id,
        name:restaurant.name,
        open_time:restaurant.open_time.to_string(),
        close_time:restaurant.close_time.to_string(),
        image_data:get_image_data(conn, restaurant.image_id)?,
        status: {
            let current_time = Local::now().time();


            if restaurant.status == "Shut Down" {
                restaurant.status
            } else if current_time < restaurant.open_time || current_time > restaurant.close_time {
                "Closed for the day".to_string()
            } else {
                restaurant.status
            }
        },
        menus:find_restaurant_menu(conn, restaurant.id)?,
        staffs:get_restaurant_staffs(conn, restaurant.id)?,
        cuisine:restaurant.cuisine
    };

    Ok(restaurant_detail)
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
pub fn change_restaurant_status(state:State<DbPool>, selected_id:i32, restaurant_status:String) -> Result<(), String> {
    let conn = &mut get_conn(&state)?;

    let mut new_status = restaurant_status.clone();

    if restaurant_status == "Open" {
        new_status = "Closed".to_string();
    } else if restaurant_status == "Closed" {
        new_status = "Open".to_string();
    }

    Restaurant::update_restaurant_status(conn, selected_id, new_status)
}

#[command]
pub fn reassign_restaurant_and_check_status(state: State<DbPool>, new_staff_id:i32, new_restaurant_id:i32) -> Result<(), String> {
    let conn = &mut get_conn(&state)?;

    let deleted_restaurant_id = Restaurant::reassign_restaurant_staff(conn, new_staff_id, new_restaurant_id)?;

    Restaurant::check_restaurant_assignment_and_update(conn, deleted_restaurant_id)
}

use crate::model::restaurant_model::{Restaurant, RestaurantDetail};
use crate::{get_conn, DbPool};
use tauri::{command, State};
#[command]
pub fn find_all_restaurant(state: State<DbPool>) -> Result<Vec<RestaurantDetail>, String> {
    let conn = &mut get_conn(&state)?;

    let restaurant_details: Vec<RestaurantDetail> = Restaurant::get_restaurant_with_menu(conn)?;

    Ok(restaurant_details)
}


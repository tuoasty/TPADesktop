use chrono::Local;
use tauri::{command, State};
use crate::{get_conn, DbPool};
use crate::handler::customer_handler::{deduct_customer_balance, get_balance};
use crate::handler::menu_handler::find_menu;
use crate::model::restaurant_order_model::{NewRestaurantOrder, RestaurantOrder};

#[command]
pub fn order_restaurant_food(state:State<DbPool>, selected_restaurant_id:i32, selected_menu_id:i32, selected_customer_id:i32, order_count:i32) -> Result<(), String>{
    let conn = &mut get_conn(&state)?;

    let menu = find_menu(conn, selected_restaurant_id, selected_menu_id)?;
    let balance = get_balance(conn, selected_customer_id)?;

    if menu.price * order_count > balance {
        return Err("Insufficient Funds".to_string())
    }

    let new_order = NewRestaurantOrder{
        customer_id:selected_customer_id,
        restaurant_id:selected_restaurant_id,
        menu_id:selected_menu_id,
        status:"New Order".to_string(),
        count:order_count,
        value:menu.price * order_count,
        time_ordered:Local::now().time()
    };

    RestaurantOrder::add_new_restaurant_order(conn, new_order)?;

    deduct_customer_balance(conn, selected_customer_id, menu.price * order_count)
}
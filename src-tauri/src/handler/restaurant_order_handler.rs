use chrono::Local;
use tauri::{command, State};
use crate::{get_conn, DbPool};
use crate::handler::customer_handler::{deduct_customer_balance, get_balance};
use crate::handler::menu_handler::find_menu;
use crate::model::restaurant_order_model::{NewRestaurantOrder, RestaurantOrder, RestaurantOrderDetail};

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

#[command]
pub fn find_restaurant_orders(state:State<DbPool>, selected_id:i32) -> Result<Vec<RestaurantOrderDetail>, String> {
    let conn = &mut get_conn(&state)?;

    let orders = RestaurantOrder::get_restaurant_orders(conn, selected_id)?;

    let order_details = orders.into_iter().map(
        |order| {
            let menu = find_menu(conn, order.restaurant_id, order.menu_id).unwrap();

            RestaurantOrderDetail {
                id:order.id,
                customer_id:order.customer_id,
                restaurant_id:order.restaurant_id,
                menu_id:order.menu_id,
                menu_name:menu.name,
                status:order.status,
                count:order.count,
                value:order.value,
                time_ordered:order.time_ordered.to_string()
            }
        }
    ).collect();

    Ok(order_details)
}

#[command]
pub fn set_order_status(state:State<DbPool>, selected_id:i32, new_status:String) -> Result<(), String> {
    let conn = &mut get_conn(&state)?;

    RestaurantOrder::set_order_status(conn, selected_id, new_status)
}
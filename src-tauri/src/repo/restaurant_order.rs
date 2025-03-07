use crate::model::restaurant_order_model::{NewRestaurantOrder, RestaurantOrder};
use diesel::prelude::*;
use crate::DbConnect;
use crate::schema::restaurant_orders::dsl::restaurant_orders;
use crate::schema::restaurant_orders::{id, status};

impl RestaurantOrder {
    pub fn add_new_restaurant_order(conn: &mut DbConnect, new_order:NewRestaurantOrder) -> Result<(), String> {
        diesel::insert_into(restaurant_orders)
            .values(new_order)
            .execute(conn)
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn get_restaurant_orders(conn: &mut DbConnect, selected_id:i32) -> Result<Vec<Self>, String> {
        restaurant_orders.filter(id.eq(&selected_id))
            .select(RestaurantOrder::as_select())
            .load(conn)
            .map_err(|e| e.to_string())
    }

    pub fn set_order_status(conn: &mut DbConnect, selected_id:i32, new_status:String) -> Result<(), String> {
        diesel::update(restaurant_orders.filter(id.eq(&selected_id)))
            .set(status.eq(new_status))
            .execute(conn)
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn get_restaurant_revenue(conn: &mut DbConnect) -> Result<Vec<RestaurantOrder>, String> {
        restaurant_orders
            .select(RestaurantOrder::as_select())
            .load(conn)
            .map_err(|e| e.to_string())
    }
}
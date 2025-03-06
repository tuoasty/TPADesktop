use crate::model::restaurant_order_model::{NewRestaurantOrder, RestaurantOrder};
use diesel::prelude::*;
use crate::DbConnect;
use crate::schema::restaurant_orders::dsl::restaurant_orders;

impl RestaurantOrder {
    pub fn add_new_restaurant_order(conn: &mut DbConnect, new_order:NewRestaurantOrder) -> Result<(), String> {
        diesel::insert_into(restaurant_orders)
            .values(new_order)
            .execute(conn)
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
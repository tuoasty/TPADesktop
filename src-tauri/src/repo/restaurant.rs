use crate::model::restaurant_model::{Restaurant, RestaurantDetail};
use crate::schema::restaurants::dsl::restaurants;
use crate::schema::restaurants::id;
use crate::DbConnect;
use diesel::prelude::*;
use crate::handler::image_handler::get_image_data;
use crate::handler::menu_handler::find_restaurant_menu;

impl Restaurant {
    pub fn get_restaurant(conn: &mut DbConnect, restaurant_id: i32) -> Result<Self, String> {
        restaurants
            .filter(id.eq(&restaurant_id))
            .first(conn)
            .map_err(|e| e.to_string())
    }
    pub fn get_all_restaurant(conn: &mut DbConnect) -> Result<Vec<Self>, String> {
        restaurants.select(Restaurant::as_select()).load(conn).map_err(|e| e.to_string())
    }

    pub fn get_restaurant_with_menu(conn: &mut DbConnect) -> Result<Vec<RestaurantDetail>, String> {
        let other_restaurants = restaurants.select(Restaurant::as_select()).load(conn).map_err(|e| e.to_string())?;

        let restaurant_details: Vec<RestaurantDetail> = other_restaurants
            .into_iter()
            .map(|restaurant| {
                let base64_image = get_image_data(conn, restaurant.image_id);
                let menus = find_restaurant_menu(conn, restaurant.id).unwrap();

                RestaurantDetail {
                    id: restaurant.id,
                    name: restaurant.name,
                    open_time: restaurant.open_time.to_string(),
                    close_time: restaurant.close_time.to_string(),
                    cuisine: restaurant.cuisine,
                    image_data: base64_image.unwrap(),
                    menus
                }
            })
            .collect();

        Ok(restaurant_details)
    }
}

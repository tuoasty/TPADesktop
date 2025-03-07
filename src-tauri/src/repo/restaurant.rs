use chrono::{Local};
use crate::model::restaurant_model::{NewRestaurant, Restaurant, RestaurantDetail};
use crate::schema::restaurants::dsl::restaurants;
use crate::schema::restaurants::{id, status};
use crate::DbConnect;
use diesel::prelude::*;
use crate::handler::image_handler::get_image_data;
use crate::handler::menu_handler::find_restaurant_menu;
use crate::handler::restaurant_assignment_handler::{check_restaurant_staff_to_open, find_staff_restaurant, get_restaurant_staffs, reassign_staff_to_restaurant};
use crate::model::ride_model::NewRide;
use crate::schema::rides::dsl::rides;

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
                let staffs = get_restaurant_staffs(conn, restaurant.id).unwrap();


                RestaurantDetail {
                    id: restaurant.id,
                    name: restaurant.name,
                    open_time: restaurant.open_time.to_string(),
                    close_time: restaurant.close_time.to_string(),
                    cuisine: restaurant.cuisine,
                    status: {
                        let current_time = Local::now().time();

                        if current_time < restaurant.open_time || current_time > restaurant.close_time {
                            "Closed".to_string()
                        } else {
                            restaurant.status
                        }
                    },
                    image_data: base64_image.unwrap(),
                    menus,
                    staffs,
                }
            })
            .collect();

        Ok(restaurant_details)
    }

    pub fn create_restaurant(conn:&mut DbConnect, new_restaurant:NewRestaurant) -> Result<(), String> {
        diesel::insert_into(restaurants)
            .values(new_restaurant)
            .execute(conn)
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn update_restaurant_status(conn:&mut DbConnect, restaurant_id:i32, new_status:String) -> Result<(), String>{
        if new_status == "Open" {
            if !check_restaurant_staff_to_open(conn, restaurant_id)? {
                return Err("Not enough staff assigned".to_string())
            }
        }

        diesel::update(restaurants.filter(id.eq(restaurant_id)))
            .set(status.eq(new_status))
            .execute(conn)
            .map_err(|e| format!("Error updating status: {}", e))?;

        Ok(())
    }

    pub fn reassign_restaurant_staff(conn: &mut DbConnect, new_staff_id:i32, new_restaurant_id:i32) -> Result<i32, String> {
        reassign_staff_to_restaurant(conn, new_staff_id, new_restaurant_id)
    }

    pub fn check_restaurant_assignment_and_update(conn: &mut DbConnect, new_restaurant_id:i32) -> Result<(), String> {
        if !check_restaurant_staff_to_open(conn, new_restaurant_id)? {
            diesel::update(restaurants.filter(id.eq(new_restaurant_id)))
                .set(status.eq("Closed".to_string()))
                .execute(conn)
                .map_err(|e| format!("Error updating status: {}", e))?;
        }

        Ok(())
    }

    pub fn get_staff_restaurant(conn: &mut DbConnect, selected_id:i32) -> Result<RestaurantDetail, String> {
        let staff_restaurant_id = find_staff_restaurant(conn, selected_id)?;

        let restaurant = Self::get_restaurant(conn, staff_restaurant_id)?;
        let image_data = get_image_data(conn, restaurant.image_id)?;
        let menus = find_restaurant_menu(conn, restaurant.id)?;
        let staffs = get_restaurant_staffs(conn, restaurant.id)?;

        let restaurant_detail = RestaurantDetail {
            id:restaurant.id,
            name:restaurant.name,
            open_time:restaurant.open_time.to_string(),
            close_time:restaurant.close_time.to_string(),
            cuisine:restaurant.cuisine,
            status:restaurant.status,
            image_data,
            menus,
            staffs
        };

        Ok(restaurant_detail)
    }
}

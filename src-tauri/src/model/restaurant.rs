use crate::DbConnect;
use crate::models::Restaurant;
use crate::schema::restaurants::dsl::restaurants;
use diesel::prelude::*;
use crate::schema::restaurants::{id};

impl Restaurant {
    pub fn get_restaurant(conn: &mut DbConnect, restaurant_id:i32) -> Result<Self, String> {
        restaurants
            .filter(id.eq(&restaurant_id))
            .first(conn)
            .map_err(|e| e.to_string())
    }
    pub fn get_all_restaurant(conn: &mut DbConnect) -> Result<Vec<Self>, String> {
        restaurants.load(conn).map_err(|e| e.to_string())
    }
}
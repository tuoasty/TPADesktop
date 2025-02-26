use tauri::{command, State};
use crate::{get_conn, DbPool};
use crate::models::{Image, Restaurant, RestaurantDetail};
use base64::encode;

#[command]
pub fn find_restaurant(state: State<DbPool>, restaurant_id:i32) -> Result<RestaurantDetail, String> {
    let conn = &mut get_conn(&state)?;

    let restaurant:Restaurant = Restaurant::get_restaurant(conn, restaurant_id)?;
    let image:Image = Image::get_image(conn, restaurant.image_id)?;

    let base64_image = format!("data:{};base64,{}", image.mime_type, encode(&image.image_data));

    let response = RestaurantDetail {
        id:restaurant.id,
        name:restaurant.name,
        open_time:restaurant.open_time.to_string(),
        close_time:restaurant.close_time.to_string(),
        cuisine:restaurant.cuisine,
        image_data:base64_image,
    };

    Ok(response)
}

#[command]
pub fn find_all_restaurant(state: State<DbPool>) -> Result<Vec<RestaurantDetail>, String> {
    let conn = &mut get_conn(&state)?;

    let restaurants: Vec<Restaurant> = Restaurant::get_all_restaurant(conn)?;

    let restaurant_details: Vec<RestaurantDetail> = restaurants
        .into_iter()
        .map(|restaurant| {
            let image:Image = Image::get_image(conn, restaurant.image_id).unwrap();
            let base64_image = format!("data:{};base64,{}", image.mime_type, encode(&image.image_data));

            RestaurantDetail {
                id:restaurant.id,
                name:restaurant.name,
                open_time:restaurant.open_time.to_string(),
                close_time:restaurant.close_time.to_string(),
                cuisine:restaurant.cuisine,
                image_data:base64_image,
            }
        }).collect();

    Ok(restaurant_details)
}
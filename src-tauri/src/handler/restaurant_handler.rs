use crate::models::{Image, NewMenu, NewMenuDetail, Restaurant, RestaurantDetail};
use crate::{get_conn, DbPool};
use base64::{decode, encode};
use tauri::{command, State};
use crate::handler::image_handler::create_image;

#[command]
pub fn find_restaurant(
    state: State<DbPool>,
    restaurant_id: i32,
) -> Result<RestaurantDetail, String> {
    let conn = &mut get_conn(&state)?;

    let restaurant: Restaurant = Restaurant::get_restaurant(conn, restaurant_id)?;
    let image: Image = Image::get_image(conn, restaurant.image_id)?;

    let base64_image = format!(
        "data:{};base64,{}",
        image.mime_type,
        encode(&image.image_data)
    );

    let response = RestaurantDetail {
        id: restaurant.id,
        name: restaurant.name,
        open_time: restaurant.open_time.to_string(),
        close_time: restaurant.close_time.to_string(),
        cuisine: restaurant.cuisine,
        image_data: base64_image,
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
            let image: Image = Image::get_image(conn, restaurant.image_id).unwrap();
            let base64_image = format!(
                "data:{};base64,{}",
                image.mime_type,
                encode(&image.image_data)
            );

            RestaurantDetail {
                id: restaurant.id,
                name: restaurant.name,
                open_time: restaurant.open_time.to_string(),
                close_time: restaurant.close_time.to_string(),
                cuisine: restaurant.cuisine,
                image_data: base64_image,
            }
        })
        .collect();

    Ok(restaurant_details)
}
#[command]
pub fn add_new_menu(state: State<DbPool>, menu: NewMenuDetail) -> Result<(), String> {
    if menu.name.is_empty() || menu.image_data.is_empty() || menu.image_name.is_empty() || menu.mime_type.is_empty() ||
        menu.restaurant_id <= 0 {
        return Err("All fields must be filled".to_string())
    }

    if menu.price <= 0 {
        return Err("Price must be greater than 0".to_string())
    }

    let conn = &mut get_conn(&state)?;
    let image_data = decode(&menu.image_data).map_err(|_| "Invalid Base encoding".to_string())?;

    let id = create_image(conn, image_data, menu.mime_type, menu.image_name)?;
    let new_menu = NewMenu {
        restaurant_id:menu.restaurant_id,
        image_id:id,
        name:menu.name,
        price:menu.price
    };

    use crate::schema::menus;
    use diesel::prelude::*;

    diesel::insert_into(menus::table)
        .values(&new_menu)
        .execute(conn)
        .map_err(|e| e.to_string())?;

    Ok(())
}

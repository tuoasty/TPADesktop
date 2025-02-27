use crate::handler::image_handler::create_image;
use crate::models::{NewMenu, NewMenuDetail};
use crate::{get_conn, DbPool};
use base64::decode;
use tauri::{command, State};

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

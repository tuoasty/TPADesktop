use base64::{Engine as _, engine::general_purpose::STANDARD};
use tauri::{command, State};
use crate::{get_conn, DbConnect, DbPool};
use crate::model::menu_model::{Menu, MenuDetail, NewMenuDetail};

pub fn find_restaurant_menu(conn: &mut DbConnect, id:i32) -> Result<Vec<MenuDetail>, String> {
    let restaurant_menus = Menu::get_restaurant_menu(conn, id)?;

    Ok(restaurant_menus)
}

pub fn find_menu(conn: &mut DbConnect, selected_restaurant_id:i32, selected_menu_id:i32) -> Result<Menu, String> {
    Menu::get_menu(conn, selected_restaurant_id, selected_menu_id)
}

#[command]
pub fn create_menu(state: State<DbPool>, menu: NewMenuDetail) -> Result<(), String> {
    if menu.name.is_empty() || menu.image_data.is_empty() || menu.image_name.is_empty() || menu.mime_type.is_empty() ||
        menu.restaurant_id <= 0 {
        return Err("All fields must be filled".to_string())
    }

    if menu.price <= 0 {
        return Err("Price must be greater than 0".to_string())
    }

    let conn = &mut get_conn(&state)?;
    let image_data = STANDARD.decode(&menu.image_data).map_err(|_| "Invalid Base encoding".to_string())?;

    Menu::create_menu(conn, menu, image_data)
}
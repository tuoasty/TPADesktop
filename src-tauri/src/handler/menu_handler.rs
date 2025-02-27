use crate::DbConnect;
use crate::handler::image_handler::get_image_data;
use crate::models::{Menu, MenuDetail};
use crate::schema::menus::dsl::menus;

pub fn find_restaurant_menu(conn: &mut DbConnect, id:i32) -> Result<Vec<MenuDetail>, String> {
    let restaurant_menus = Menu::get_restaurant_menu(conn, id)?;

    Ok(restaurant_menus)
}
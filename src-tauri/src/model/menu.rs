use crate::DbConnect;
use crate::models::{Menu, MenuDetail};
use crate::schema::menus::dsl::menus;
use crate::schema::menus::restaurant_id;
use diesel::prelude::*;
use crate::handler::image_handler::get_image_data;

impl Menu {
    pub fn get_restaurant_menu(conn: &mut DbConnect, id:i32) -> Result<Vec<MenuDetail>, String> {
        let other_menus =
            menus.filter(restaurant_id.eq(&id))
                .select(Menu::as_select())
                .load(conn)
                .map_err(|e| e.to_string())?;

        let menu_details: Vec<MenuDetail> = other_menus
            .into_iter()
            .map(|menu| {
                let base64_image = get_image_data(conn, menu.image_id);

                MenuDetail {
                    id: menu.id,
                    name: menu.name,
                    price: menu.price,
                    image_data: base64_image.unwrap(),
                }
            })
            .collect();

        Ok(menu_details)

    }
}
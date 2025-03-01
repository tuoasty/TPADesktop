use crate::DbConnect;
use crate::model::menu_model::{Menu, MenuDetail, NewMenu, NewMenuDetail};
use crate::schema::menus::dsl::menus;
use crate::schema::menus::restaurant_id;
use diesel::prelude::*;
use crate::handler::image_handler::{create_image, get_image_data};

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

    pub fn create_menu(conn: &mut DbConnect, menu_detail:NewMenuDetail, image_data:Vec<u8>) -> Result<(), String> {
        let image_id = create_image(conn, image_data, menu_detail.mime_type, menu_detail.image_name)?;

        let new_menu = NewMenu {
            restaurant_id:menu_detail.restaurant_id,
            image_id,
            name:menu_detail.name,
            price:menu_detail.price
        };

        use crate::schema::menus;
        use diesel::prelude::*;

        diesel::insert_into(menus::table)
            .values(&new_menu)
            .execute(conn)
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
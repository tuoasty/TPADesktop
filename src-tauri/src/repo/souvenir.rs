use crate::DbConnect;
use crate::model::souvenir_model::{Souvenir, SouvenirDetail};
use crate::schema::souvenirs::dsl::souvenirs;
use diesel::prelude::*;
use crate::handler::image_handler::{get_image_data, remove_image};
use crate::schema::souvenirs::{id, image_id, store_id};
impl Souvenir{
    pub fn get_souvenir_of_store(conn: &mut DbConnect, selected_id:i32) -> Result<Vec<SouvenirDetail>, String> {
        let other_souvenirs =
            souvenirs.filter(store_id.eq(&selected_id))
                .select(Souvenir::as_select())
                .load(conn)
                .map_err(|e| e.to_string())?;

        let souvenir_details: Vec<SouvenirDetail> = other_souvenirs
            .into_iter()
            .map(|souvenir| {
                let base64_image = get_image_data(conn, souvenir.image_id);

                SouvenirDetail {
                    id: souvenir.id,
                    name: souvenir.name,
                    price: souvenir.price,
                    description : souvenir.description,
                    image_data: base64_image.unwrap(),
                }
            })
            .collect();

        Ok(souvenir_details)
    }

    pub fn remove_souvenir_and_image(conn: &mut DbConnect, selected_id:i32) -> Result<(), String> {
        let selected_image_id:i32 = souvenirs
            .filter(id.eq(selected_id))
            .select(image_id)
            .first(conn).unwrap();

        diesel::delete(souvenirs.filter(id.eq(selected_id)))
            .execute(conn)
            .map_err(|e| format!("Failed to delete souvenir: {}", e))?;

        remove_image(conn, selected_image_id)
    }
}
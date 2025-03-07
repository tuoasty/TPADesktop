use crate::DbConnect;
use crate::model::souvenir_model::{NewSouvenir, NewSouvenirDetail, Souvenir, SouvenirDetail};
use crate::schema::souvenirs::dsl::souvenirs;
use diesel::prelude::*;
use crate::handler::image_handler::{create_image, get_image_data, remove_image};
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

    pub fn create_souvenir(conn: &mut DbConnect, souvenir_detail: NewSouvenirDetail, image_data:Vec<u8>) -> Result<(), String> {
        let new_image_id = create_image(conn, image_data, souvenir_detail.mime_type, souvenir_detail.image_name)?;

        let new_souvenir = NewSouvenir {
            name:souvenir_detail.name,
            store_id:souvenir_detail.store_id,
            price:souvenir_detail.price,
            image_id:new_image_id,
            description:souvenir_detail.description
        };

        diesel::insert_into(souvenirs)
            .values(new_souvenir)
            .execute(conn)
            .map_err(|e| e.to_string())?;

        Ok(())
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

    pub fn get_souvenir(conn: &mut DbConnect, selected_id:i32) -> Result<Souvenir, String> {
        souvenirs.filter(id.eq(&selected_id))
            .first(conn)
            .map_err(|e| e.to_string())
    }
}
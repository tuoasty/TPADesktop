use crate::DbConnect;
use crate::models::{Souvenir, SouvenirDetail};
use crate::schema::souvenirs::dsl::souvenirs;
use diesel::prelude::*;
use crate::handler::image_handler::get_image_data;
use crate::schema::souvenirs::store_id;
impl Souvenir{
    pub fn get_souvenir_of_store(conn: &mut DbConnect, id:i32) -> Result<Vec<SouvenirDetail>, String> {
        let other_souvenirs =
            souvenirs.filter(store_id.eq(&id))
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
}
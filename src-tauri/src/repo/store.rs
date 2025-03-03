use crate::DbConnect;
use crate::model::store_model::{Store, StoreDetail};
use crate::schema::stores::dsl::stores;
use diesel::prelude::*;
use crate::handler::image_handler::get_image_data;
use crate::handler::souvenir_handler::find_store_souvenir;
use crate::handler::store_assignment_handler::get_store_staffs;
use crate::model::souvenir_model::SouvenirDetail;
use crate::model::staff_model::StaffDetail;
use crate::schema::stores::{id, status};

impl Store {
    pub fn get_all_stores(conn: &mut DbConnect) -> Result<Vec<StoreDetail>, String> {
        let other_stores = stores.select(Store::as_select()).load(conn).map_err(|e| e.to_string())?;

        let store_details: Vec<StoreDetail> = other_stores
            .into_iter()
            .map(|store| {
                let base64_image = get_image_data(conn, store.image_id).unwrap();
                let souvenir_list: Vec<SouvenirDetail> = find_store_souvenir(conn, store.id).unwrap();
                let staffs:Vec<StaffDetail> = get_store_staffs(conn, store.id).unwrap();

                StoreDetail {
                    id: store.id,
                    name: store.name,
                    open_time: store.open_time.to_string(),
                    close_time: store.close_time.to_string(),
                    status: store.status,
                    image_data: base64_image,
                    souvenirs:souvenir_list,
                    staffs
                }
            })
            .collect();

        Ok(store_details)
    }

    pub fn update_store_status(conn:&mut DbConnect, store_id:i32, new_status:String) -> Result<(), String>{
        diesel::update(stores.filter(id.eq(store_id)))
            .set(status.eq(new_status))
            .execute(conn)
            .map_err(|e| format!("Error updating status: {}", e))?;

        Ok(())
    }
}
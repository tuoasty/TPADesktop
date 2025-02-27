use base64::encode;
use tauri::{command, State};
use crate::{get_conn, DbPool};
use crate::models::{Image, Souvenir, Store, StoreDetail};

#[command]
pub fn find_all_store(state: State<DbPool>) -> Result<Vec<StoreDetail>, String> {
    let conn = &mut get_conn(&state)?;

    let stores: Vec<Store> = Store::get_all_stores(conn)?;


    let store_details: Vec<StoreDetail> = stores
        .into_iter()
        .map(|store| {
            let image: Image = Image::get_image(conn, store.image_id).unwrap();
            let base64_image = format!(
                "data:{};base64,{}",
                image.mime_type,
                encode(&image.image_data)
            );

            let souvenir_list: Vec<Souvenir> = Souvenir::get_souvenir_of_store(conn, store.id).unwrap();

            StoreDetail {
                id: store.id,
                name: store.name,
                open_time: store.open_time.to_string(),
                close_time: store.close_time.to_string(),
                image_data: base64_image,
                souvenirs:souvenir_list
            }
        })
        .collect();

    Ok(store_details)
}

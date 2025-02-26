use diesel::RunQueryDsl;
use tauri::State;
use crate::{get_conn, DbPool};
use crate::models::{Image, NewImage};
use crate::schema::images::dsl::images;

pub fn create_image(state: State<DbPool>, image:Vec<u8>, mime:String, name: String) -> Result<i32, String> {
    use crate::schema::images::dsl::*;
    let conn = &mut get_conn(&state)?;

    let new_image = NewImage {
        image_data:image,
        mime_type:mime,
        filename:name,
    };

    diesel::insert_into(images)
        .values(&new_image)
        .returning(id)
        .get_result(conn)
        .map_err(|e| e.to_string())
}

pub fn get_image(state: State<DbPool>, image_id:i32) -> Result<Vec<u8>, String> {
    let conn = &mut get_conn(&state)?;

    let image = Image::get_image(conn, image_id).map_err(|e| e.to_string())?;

    Ok(image.image_data)
}


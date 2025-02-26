use std::path::Path;
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


use base64::encode;
use crate::models::{Image, NewImage};
use crate::DbConnect;
use diesel::RunQueryDsl;

pub fn create_image(
    conn: &mut DbConnect,
    image: Vec<u8>,
    mime: String,
    name: String,
) -> Result<i32, String> {
    use crate::schema::images::dsl::*;

    let new_image = NewImage {
        image_data: image,
        mime_type: mime,
        filename: name,
    };

    diesel::insert_into(images)
        .values(&new_image)
        .returning(id)
        .get_result(conn)
        .map_err(|e| e.to_string())
}

pub fn get_image_data(conn: &mut DbConnect, id:i32) -> Result<String, String> {
    let image: Image = Image::get_image(conn, id)?;
    let base64_image = format!(
        "data:{};base64,{}",
        image.mime_type,
        encode(&image.image_data)
    );

    Ok(base64_image)
}
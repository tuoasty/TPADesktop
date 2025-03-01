use base64::{Engine as _, engine::general_purpose::STANDARD};
use crate::models::{Image, NewImage};
use crate::DbConnect;

pub fn create_image(
    conn: &mut DbConnect,
    image: Vec<u8>,
    mime: String,
    name: String,
) -> Result<i32, String> {
    let new_image = NewImage {
        image_data: image,
        mime_type: mime,
        filename: name,
    };

    Image::create_image(conn, new_image)
}

pub fn get_image_data(conn: &mut DbConnect, image_id:i32) -> Result<String, String> {
    let image: Image = Image::get_image(conn, image_id)?;
    let base64_image = format!(
        "data:{};base64,{}",
        image.mime_type,
        STANDARD.encode(&image.image_data)
    );

    Ok(base64_image)
}

pub fn remove_image(conn: &mut DbConnect, image_id:i32) -> Result<(), String> {
    Image::remove_image(conn, image_id)
}
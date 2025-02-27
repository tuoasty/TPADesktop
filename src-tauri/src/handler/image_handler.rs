use crate::models::NewImage;
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

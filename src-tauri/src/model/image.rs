use crate::models::{Image, NewImage};
use crate::schema::images::dsl::images;
use crate::schema::images::id;
use crate::DbConnect;
use diesel::prelude::*;
use diesel::ExpressionMethods;

impl Image {
    pub fn get_image(conn: &mut DbConnect, image_id: i32) -> Result<Self, String> {
        images
            .filter(id.eq(image_id))
            .first(conn)
            .map_err(|e| e.to_string())
    }

    pub fn create_image(conn: &mut DbConnect, new_image: NewImage) -> Result<i32, String>{
        diesel::insert_into(images)
            .values(&new_image)
            .returning(id)
            .get_result(conn)
            .map_err(|e| e.to_string())
    }

    pub fn remove_image(conn: &mut DbConnect, selected_id:i32) -> Result<(), String> {
        diesel::delete(
            images.filter(id.eq(selected_id)))
            .execute(conn)
            .map_err(|e| format!("Failed to delete image: {}", e))?;

        Ok(())
    }
}

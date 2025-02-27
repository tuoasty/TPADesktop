use crate::models::Image;
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
}

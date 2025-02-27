use crate::DbConnect;
use crate::models::{Souvenir};
use crate::schema::souvenirs::dsl::souvenirs;
use diesel::prelude::*;
use crate::schema::souvenirs::store_id;
impl Souvenir{
    pub fn get_souvenir_of_store(conn: &mut DbConnect, id:i32) -> Result<Vec<Self>, String> {
        souvenirs
            .filter(store_id.eq(&id))
            .select(Souvenir::as_select())
            .load(conn)
            .map_err(|e| e.to_string())
    }
}
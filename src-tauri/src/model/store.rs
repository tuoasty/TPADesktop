use crate::DbConnect;
use crate::models::Store;
use crate::schema::stores::dsl::stores;
use diesel::prelude::*;

impl Store {
    pub fn get_all_stores(conn: &mut DbConnect) -> Result<Vec<Self>, String> {
        stores.select(Store::as_select()).load(conn).map_err(|e| e.to_string())
    }
}
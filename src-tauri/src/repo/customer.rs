use crate::DbConnect;
use crate::model::customer_model::Customer;
use diesel::prelude::*;
use crate::schema::customers::dsl::customers;
use crate::schema::customers::{id, name};

impl Customer {
    pub fn get_customer_name(conn: &mut DbConnect, selected_id:i32) -> Result<String, String> {
        customers.filter(id.eq(&selected_id))
            .select(name)
            .first(conn)
            .map_err(|e| e.to_string())
    }
}
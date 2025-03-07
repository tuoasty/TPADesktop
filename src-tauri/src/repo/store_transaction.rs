use crate::DbConnect;
use crate::model::store_transaction_model::{NewStoreTransaction, StoreTransaction};
use diesel::prelude::*;
use crate::schema::store_transactions::dsl::store_transactions;
use crate::schema::store_transactions::store_id;

impl StoreTransaction {
    pub fn create_store_transaction(conn: &mut DbConnect, new_transaction:NewStoreTransaction) -> Result<(), String> {
        diesel::insert_into(store_transactions)
            .values(new_transaction)
            .execute(conn)
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn get_store_transaction(conn: &mut DbConnect, selected_id:i32) -> Result<Vec<StoreTransaction>, String> {
        store_transactions.filter(store_id.eq(&selected_id))
            .select(StoreTransaction::as_select())
            .load(conn)
            .map_err(|e| e.to_string())
    }
}
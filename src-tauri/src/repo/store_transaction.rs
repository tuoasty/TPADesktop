use crate::DbConnect;
use crate::model::store_transaction_model::{NewStoreTransaction, StoreTransaction};
use diesel::prelude::*;
use crate::schema::store_transactions::dsl::store_transactions;

impl StoreTransaction {
    pub fn create_store_transaction(conn: &mut DbConnect, new_transaction:NewStoreTransaction) -> Result<(), String> {
        diesel::insert_into(store_transactions)
            .values(new_transaction)
            .execute(conn)
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
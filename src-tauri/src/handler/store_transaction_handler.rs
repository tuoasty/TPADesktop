use crate::DbConnect;
use crate::model::store_transaction_model::{NewStoreTransaction, StoreTransaction};

pub fn create_new_store_transaction(conn: &mut DbConnect, customer_id:i32, store_id:i32, souvenir_id:i32, count:i32) -> Result<(), String> {
    let transaction = NewStoreTransaction {
        customer_id,
        store_id,
        souvenir_id,
        count
    };

    StoreTransaction::create_store_transaction(conn, transaction)
}
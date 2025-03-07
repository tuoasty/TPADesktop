use chrono::Local;
use crate::{DbConnect};
use crate::model::store_transaction_model::{NewStoreTransaction, StoreTransaction, StoreTransactionDetail};

pub fn create_new_store_transaction(conn: &mut DbConnect, customer_id:i32, store_id:i32, souvenir_id:i32, count:i32, value:i32) -> Result<(), String> {
    let transaction = NewStoreTransaction {
        customer_id,
        store_id,
        souvenir_id,
        count,
        value,
        time_ordered:Local::now().time()
    };

    StoreTransaction::create_store_transaction(conn, transaction)
}

pub fn get_store_transaction(conn: &mut DbConnect, selected_id:i32) -> Result<Vec<StoreTransaction>, String> {
    StoreTransaction::get_store_transaction(conn, selected_id)
}
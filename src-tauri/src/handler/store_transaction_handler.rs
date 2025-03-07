use chrono::Local;
use tauri::{command, State};
use crate::{get_conn, DbConnect, DbPool};
use crate::model::store_transaction_model::{NewStoreTransaction, StoreRevenue, StoreTransaction, StoreTransactionDetail};

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

#[command]
pub fn find_store_revenue(state:State<DbPool>) -> Result<Vec<StoreRevenue>, String> {
    let conn = &mut get_conn(&state)?;

    let store_revenue = StoreTransaction::get_store_revenue(conn)?;
    let revenue_details = store_revenue.into_iter().map(
        |revenue| {
            StoreRevenue {
                id:revenue.id,
                time:revenue.time_ordered.to_string(),
                value:revenue.value,
                store_id:revenue.store_id,
                souvenir_id:revenue.souvenir_id
            }
        }
    ).collect();

    Ok(revenue_details)
}
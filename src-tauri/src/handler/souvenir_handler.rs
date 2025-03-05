use tauri::{command, State};
use crate::{get_conn, CurrentCustomer, DbConnect, DbPool};
use crate::handler::customer_handler::{deduct_customer_balance, get_customer_balance, get_customer_id};
use crate::handler::store_transaction_handler::create_new_store_transaction;
use crate::model::souvenir_model::{Souvenir, SouvenirDetail};
pub fn find_store_souvenir(conn: &mut DbConnect,selected_id:i32) -> Result<Vec<SouvenirDetail>, String> {
    let store_souvenirs = Souvenir::get_souvenir_of_store(conn, selected_id)?;

    Ok(store_souvenirs)
}

#[command]
pub fn remove_souvenir(state:State<DbPool>, selected_id:i32) -> Result<(), String> {
    let conn = &mut get_conn(&state)?;

    Souvenir::remove_souvenir_and_image(conn, selected_id)
}

#[command]
pub fn purchase_souvenir(state:State<DbPool>, curr_customer:State<CurrentCustomer>, souvenir_id:i32, souvenir_count:i32) -> Result<(), String> {
    if souvenir_count <= 0 {
        return Err("Must purchase at least 1 souvenir".to_string())
    };

    eprintln!("Test");

    let conn = &mut get_conn(&state)?;

    let balance = get_customer_balance(&curr_customer).map_err(|e| e.to_string())?;
    let souvenir = Souvenir::get_souvenir(conn, souvenir_id)?;

    if souvenir.price * souvenir_count > balance {
        return Err("Balance insufficient".to_string())
    };

    let customer_id = get_customer_id(&curr_customer).map_err(|e| e.to_string())?;

    deduct_customer_balance(conn, customer_id, souvenir.price * souvenir_count)?;

    create_new_store_transaction(conn, customer_id, souvenir.store_id, souvenir.id, souvenir_count)

}
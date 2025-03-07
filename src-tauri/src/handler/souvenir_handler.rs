use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use tauri::{command, State};
use crate::{get_conn, CurrentCustomer, DbConnect, DbPool};
use crate::handler::customer_handler::{deduct_customer_balance, get_customer_balance, get_customer_id};
use crate::handler::store_transaction_handler::create_new_store_transaction;
use crate::model::souvenir_model::{NewSouvenir, NewSouvenirDetail, Souvenir, SouvenirDetail};
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
pub fn create_souvenir(state:State<DbPool>, souvenir:NewSouvenirDetail) -> Result<(), String> {
    if souvenir.name.is_empty() || souvenir.description.is_empty() || souvenir.image_name.is_empty() ||
        souvenir.mime_type.is_empty() || souvenir.image_data.is_empty() || souvenir.store_id <= 0{
        return Err("All fields must be filled".to_string())
    };

    if souvenir.price <= 0 {
        return Err("Price must be greater than 0".to_string())
    };

    let conn = &mut get_conn(&state)?;
    let image_data = STANDARD.decode(&souvenir.image_data).map_err(|_| "Invalid Base encoding".to_string())?;

    Souvenir::create_souvenir(conn, souvenir, image_data)
}

#[command]
pub fn purchase_souvenir(state:State<DbPool>, curr_customer:State<CurrentCustomer>, souvenir_id:i32, souvenir_count:i32) -> Result<(), String> {
    if souvenir_count <= 0 {
        return Err("Must purchase at least 1 souvenir".to_string())
    };

    let conn = &mut get_conn(&state)?;

    let balance = get_customer_balance(&curr_customer).map_err(|e| e.to_string())?;
    let souvenir = Souvenir::get_souvenir(conn, souvenir_id)?;

    if souvenir.price * souvenir_count > balance {
        return Err("Balance insufficient".to_string())
    };

    let customer_id = get_customer_id(&curr_customer).map_err(|e| e.to_string())?;

    deduct_customer_balance(conn, customer_id, souvenir.price * souvenir_count)?;

    create_new_store_transaction(conn, customer_id, souvenir.store_id, souvenir.id, souvenir_count, souvenir.price * souvenir_count)

}

pub fn find_souvenir(conn: &mut DbConnect, selected_id:i32) -> Result<Souvenir, String> {
    Souvenir::get_souvenir(conn, selected_id)
}
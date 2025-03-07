use tauri::{command, State};
use crate::{get_conn, CurrentCustomer, DbConnect, DbPool};
use crate::model::customer_model::{Customer, NewCustomer};
pub fn find_customer_name(conn: &mut DbConnect, selected_id:i32) -> Result<String, String> {
    Customer::get_customer_name(conn, selected_id)
}

#[command]
pub async fn verify_customer_login(current_customer:State<'_, CurrentCustomer>) -> Result<bool, String> {
    let curr_customer = current_customer.0.lock().unwrap();

    if curr_customer.is_none(){
        return Ok(false);
    }
    Ok(true)
}

#[command]
pub fn login_customer(state:State<DbPool>, current_customer:State<CurrentCustomer>, id:i32, name:String) -> Result<String, String> {
    let conn = &mut get_conn(&state)?;

    let customer = Customer::get_customer(conn, id)?;

    if customer.name.eq(&name) {
        *current_customer.0.lock().unwrap() = Some((customer.id, customer.name.clone(), customer.balance));
        Ok(format!("Welcome {}", &customer.name))
    } else {
        Err("Incorrect credential".to_string())
    }
}

#[command]
pub fn get_current_customer(current_customer:State<CurrentCustomer>) -> Result<Option<(i32, String, i32)>, String> {
    let customer = current_customer.0.lock().expect("Access error").clone();

    Ok(customer)
}

#[command]
pub fn logout_customer(current_customer:State<CurrentCustomer>) -> Result<String, String> {
    let mut lock = current_customer.0.lock().unwrap();
    *lock = None;
    Ok("Successfully logout".to_string())
}

pub fn get_customer_balance(current_customer:&CurrentCustomer) -> Result<i32, String> {
    let curr_customer = current_customer.0.lock()
        .map_err(|_| "Failed to acquire lock".to_string())?
        .clone()
        .ok_or("No customer found".to_string())?;

    Ok(curr_customer.2)
}

#[command]
pub fn get_all_customer(state:State<DbPool>) -> Result<Vec<Customer>, String> {
    let conn = &mut get_conn(&state)?;

    Customer::get_all_customer(conn)
}

pub fn get_balance(conn: &mut DbConnect, selected_id:i32) -> Result<i32, String> {
    Customer::get_balance(conn, selected_id)
}

pub fn get_customer_id(current_customer:&CurrentCustomer) -> Result<i32, String> {
    let curr_customer = current_customer.0.lock()
        .map_err(|_| "Failed to acquire lock".to_string())?
        .clone()
        .ok_or("No customer found".to_string())?;

    Ok(curr_customer.0)
}

pub fn deduct_customer_balance(conn: &mut DbConnect, customer_id:i32, value:i32) -> Result<(), String> {
    Customer::deduct_customer_balance(conn, customer_id, value)
}

#[command]
pub fn create_customer_account(state:State<DbPool>, data:NewCustomer) -> Result<i32, String> {
    let conn = &mut get_conn(&state)?;
    Customer::create_customer(conn, data)
}
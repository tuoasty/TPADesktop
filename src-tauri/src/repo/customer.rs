use crate::DbConnect;
use crate::model::customer_model::{Customer, NewCustomer};
use diesel::prelude::*;
use crate::schema::customers::dsl::customers;
use crate::schema::customers::{balance, id, name};

impl Customer {
    pub fn get_customer_name(conn: &mut DbConnect, selected_id:i32) -> Result<String, String> {
        customers.filter(id.eq(&selected_id))
            .select(name)
            .first(conn)
            .map_err(|e| e.to_string())
    }

    pub fn get_customer(conn:&mut DbConnect, cust_id:i32) -> Result<Self, String> {
        customers.filter(id.eq(&cust_id))
            .first(conn)
            .map_err(|e| e.to_string())
    }

    pub fn deduct_customer_balance(conn: &mut DbConnect, customer_id:i32, value:i32) -> Result<(), String> {
        let current_balance:i32 = customers.filter(id.eq(&customer_id))
            .select(balance)
            .first(conn)
            .map_err(|e| e.to_string())?;

        let new_balance = current_balance - value;

        diesel::update(customers.filter(id.eq(&customer_id))).set(balance.eq(&new_balance)).execute(conn).map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn get_balance(conn: &mut DbConnect, selected_id:i32) -> Result<i32, String> {
        customers.filter(id.eq(&selected_id))
            .select(balance)
            .first(conn)
            .map_err(|e| e.to_string())
    }

    pub fn create_customer(conn: &mut DbConnect, data:NewCustomer) -> Result<i32, String> {
        diesel::insert_into(customers)
            .values(&data)
            .returning(id)
            .get_result::<i32>(conn)
            .map_err(|e| e.to_string())
    }

}
use crate::DbConnect;
use crate::model::customer_model::Customer;
pub fn find_customer_name(conn: &mut DbConnect, selected_id:i32) -> Result<String, String> {
    Customer::get_customer_name(conn, selected_id)
}
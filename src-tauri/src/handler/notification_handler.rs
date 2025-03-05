use tauri::{command, State};
use crate::{get_conn, DbConnect, DbPool};
use crate::model::notification_model::{NewNotification, Notification};

pub fn create_customer_notification(conn: &mut DbConnect, selected_id:i32, new_message:String) -> Result<(), String> {
    let new_notification = NewNotification {
        customer_id:selected_id,
        message:new_message
    };

    Notification::create_notification(conn, new_notification)
}

#[command]
pub fn broadcast_message(state:State<DbPool>, new_message:String) -> Result<(), String> {
    let conn = &mut get_conn(&state)?;
    let new_notification = NewNotification {
        customer_id:0,
        message:new_message
    };

    Notification::create_notification(conn, new_notification)
}

#[command]
pub fn find_customer_notifications(state:State<DbPool>, selected_id:i32) -> Result<Vec<Notification>, String> {
    let conn = &mut get_conn(&state)?;

    Notification::get_customer_notification(conn, selected_id)

}
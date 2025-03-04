use diesel::{ExpressionMethods, RunQueryDsl};
use crate::DbConnect;
use crate::model::notification_model::{NewNotification, Notification};
use crate::schema::notifications::dsl::notifications;
use diesel::prelude::*;
use crate::schema::notifications::customer_id;

impl Notification {
    pub fn create_notification(conn: &mut DbConnect, new_notification: NewNotification) -> Result<(),String> {
        diesel::insert_into(notifications)
            .values(new_notification)
            .execute(conn)
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn get_customer_notification(conn:&mut DbConnect, selected_id:i32) -> Result<Vec<Self>, String> {
        notifications.filter(customer_id.eq(&selected_id))
            .select(Notification::as_select())
            .load(conn)
            .map_err(|e| e.to_string())
    }
}
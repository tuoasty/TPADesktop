use crate::model::notification_model::{NewNotification, Notification};
use crate::schema::notifications::customer_id;
use crate::schema::notifications::dsl::notifications;
use crate::DbConnect;
use diesel::prelude::*;
use diesel::{ExpressionMethods, RunQueryDsl};

impl Notification {
    pub fn create_notification(
        conn: &mut DbConnect,
        new_notification: NewNotification,
    ) -> Result<(), String> {
        diesel::insert_into(notifications)
            .values(new_notification)
            .execute(conn)
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn get_customer_notification(
        conn: &mut DbConnect,
        selected_id: i32,
    ) -> Result<Vec<Self>, String> {
        let customer_notifications = notifications
            .filter(customer_id.eq(&selected_id))
            .select(Notification::as_select());

        let broadcast_notifications = notifications
            .filter(customer_id.eq(0))
            .select(Notification::as_select());

        customer_notifications.union(broadcast_notifications)
            .load(conn)
            .map_err(|e| e.to_string())
    }
}

use diesel::ExpressionMethods;
use crate::DbConnect;
use crate::model::ride_queue_model::{NewRideQueue, RideQueue};
use crate::schema::ride_queues::dsl::ride_queues;
use crate::schema::ride_queues::{customer_id, ride_id, status, time_joined};
use diesel::prelude::*;
use crate::handler::customer_handler::deduct_customer_balance;

impl RideQueue {
    pub fn get_ride_queues(conn: &mut DbConnect, selected_id:i32) -> Result<Vec<RideQueue>, String> {
        ride_queues
            .filter(ride_id.eq(&selected_id))
            .order(time_joined.asc())
            .load(conn)
            .map_err(|e| e.to_string())
    }

    pub fn add_ride_queue(conn: &mut DbConnect, customer:NewRideQueue) -> Result<(), String> {
        diesel::insert_into(ride_queues)
            .values(customer)
            .execute(conn)
            .map_err(|e| e.to_string())?;


        Ok(())
    }

    pub fn dequeue_ride(conn: &mut DbConnect, selected_ride_id:i32, selected_customer_id:i32) -> Result<(), String> {
            diesel::update(ride_queues)
                .filter(customer_id.eq(&selected_customer_id)
                    .and(status.eq("Waiting in Line".to_string()))
                    .and(ride_id.eq(&selected_ride_id)))
            .set(status.eq("Completed"))
            .execute(conn)
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
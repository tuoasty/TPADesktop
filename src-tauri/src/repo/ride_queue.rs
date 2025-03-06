use diesel::ExpressionMethods;
use crate::DbConnect;
use crate::model::ride_queue_model::RideQueue;
use crate::schema::ride_queues::dsl::ride_queues;
use crate::schema::ride_queues::{ride_id, time_joined};
use diesel::prelude::*;
use diesel::query_dsl::methods::OrderDsl;

impl RideQueue {
    pub fn get_ride_queues(conn: &mut DbConnect, selected_id:i32) -> Result<Vec<RideQueue>, String> {
        ride_queues
            .filter(ride_id.eq(&selected_id))
            .load(conn)
            .order(time_joined.asc())
            .map_err(|e| e.to_string())
    }
}
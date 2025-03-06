use tauri::{command, State};
use crate::{get_conn, DbPool};
use crate::handler::customer_handler::find_customer_name;
use crate::handler::ride_handler::find_ride;
use crate::model::ride_queue_model::{RideQueue, RideQueueDetail};

#[command]
pub fn find_ride_queue(state:State<DbPool>, selected_id:i32) -> Result<Vec<RideQueueDetail>, String> {
    let conn = &mut get_conn(&state)?;
    let queues = RideQueue::get_ride_queues(conn, selected_id)?;

    let queue_details:Vec<RideQueueDetail> = queues.into_iter().map(
        |queue|{
            let ride = find_ride(conn, queue.ride_id)?;
            let customer = find_customer_name(conn, queue.customer_id)?;

            RideQueueDetail {
                id:queue.id,
                ride_id:queue.ride_id,
                ride_name:ride.name,
                customer_id:queue.customer_id,
                customer_name:customer,
                time_joined:queue.time_joined.to_string(),
                status:queue.status
            }
        }).collect();
    Ok(queue_details)
}
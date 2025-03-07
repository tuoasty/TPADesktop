use chrono::Local;
use tauri::{command, State};
use crate::{get_conn, DbPool};
use crate::handler::customer_handler::{deduct_customer_balance, find_customer_name, get_balance};
use crate::handler::ride_handler::{find_ride, find_ride_price};
use crate::model::ride_queue_model::{NewRideQueue, RideQueue, RideQueueDetail, RideRevenue};

#[command]
pub fn find_ride_queue(state:State<DbPool>, selected_id:i32) -> Result<Vec<RideQueueDetail>, String> {
    let conn = &mut get_conn(&state)?;
    let queues = RideQueue::get_ride_queues(conn, selected_id)?;

    let queue_details:Vec<RideQueueDetail> = queues.into_iter().map(
        |queue|{
            let ride = find_ride(conn, queue.ride_id).unwrap();
            let customer = find_customer_name(conn, queue.customer_id).unwrap();

            RideQueueDetail {
                id:queue.id,
                ride_id:queue.ride_id,
                ride_name:ride.name,
                customer_id:queue.customer_id,
                customer_name:customer,
                time_joined:queue.time_joined.to_string(),
                status:queue.status,
                value:queue.value
            }
        }).collect();
    Ok(queue_details)
}

#[command]
pub fn add_customer_to_ride_queue(state:State<DbPool>, selected_ride_id:i32, selected_customer_id:i32) -> Result<(), String> {
    let conn = &mut get_conn(&state)?;

    let customer_balance = get_balance(conn, selected_customer_id)?;
    let ride_price = find_ride_price(conn, selected_ride_id)?;

    if customer_balance < ride_price {
        return Err("Insufficient Funds".to_string())
    }

    let ride_queue = NewRideQueue {
        ride_id:selected_ride_id,
        customer_id:selected_customer_id,
        time_joined:Local::now().time(),
        status:"Waiting in Line".to_string(),
        value:ride_price
    };

    RideQueue::add_ride_queue(conn, ride_queue)?;

    deduct_customer_balance(conn, selected_customer_id, ride_price)
}

#[command]
pub fn dequeue_customer_from_ride(state:State<DbPool>, selected_ride_id:i32, selected_customer_id:i32) -> Result<(), String> {
    let conn = &mut get_conn(&state)?;

    RideQueue::dequeue_ride(conn, selected_ride_id, selected_customer_id)
}

#[command]
pub fn find_ride_revenue(state:State<DbPool>) -> Result<Vec<RideRevenue>, String> {
    let conn = &mut get_conn(&state)?;
    let ride_queue = RideQueue::get_ride_revenue(conn)?;
    let ride_revenue = ride_queue.into_iter().map(
        |ride| {
            RideRevenue {
                id:ride.id,
                time:ride.time_joined.to_string(),
                value:ride.value,
                ride_id:ride.ride_id,
            }
        }
    ).collect();

    Ok(ride_revenue)
}
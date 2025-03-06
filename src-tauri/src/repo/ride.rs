use chrono::Local;
use crate::DbConnect;
use crate::handler::image_handler::get_image_data;
use crate::model::ride_model::{NewRide, Ride, RideDetail};
use diesel::prelude::*;
use crate::handler::ride_assignment_handler::{check_ride_staff_to_open, find_staff_ride, get_ride_staffs, reassign_staff_to_ride};
use crate::schema::rides::dsl::rides;
use crate::schema::rides::{id, price, status};

impl Ride {
    pub fn get_all_ride(conn:&mut DbConnect) -> Result<Vec<RideDetail>, String> {
        let other_rides = rides.select(Ride::as_select()).load(conn).map_err(|e| e.to_string())?;

        let ride_details: Vec<RideDetail> = other_rides
            .into_iter()
            .map(|ride| {
                let base64_image = get_image_data(conn, ride.image_id);
                let staffs = get_ride_staffs(conn, ride.id).unwrap();

                RideDetail {
                    id:ride.id,
                    name:ride.name,
                    open_time:ride.open_time.to_string(),
                    close_time:ride.close_time.to_string(),
                    price:ride.price,
                    image_data:base64_image.unwrap(),
                    status: {
                        let current_time = Local::now().time();

                        if current_time < ride.open_time || current_time > ride.close_time {
                            "Closed".to_string()
                        } else {
                            ride.status
                        }
                    },
                    staffs
                }
            })
            .collect();

        Ok(ride_details)
    }

    pub fn get_ride(conn:&mut DbConnect, ride_id:i32) -> Result<Ride, String> {
        rides
            .filter(id.eq(&ride_id))
            .first(conn)
            .map_err(|e| e.to_string())
    }

    pub fn get_staff_ride(conn:&mut DbConnect, selected_id:i32) -> Result<RideDetail, String> {
        let staff_ride = find_staff_ride(conn, selected_id)?;

        let ride = rides.filter(id.eq(&staff_ride))
            .select(Ride::as_select())
            .first(conn)
            .map_err(|e| e.to_string())?;
        let image_data = get_image_data(conn, ride.image_id)?;
        let staffs = get_ride_staffs(conn, ride.id)?;

        let ride_detail = RideDetail {
            id:ride.id,
            name:ride.name,
            open_time:ride.open_time.to_string(),
            close_time:ride.close_time.to_string(),
            price:ride.price,
            image_data,
            status:ride.status,
            staffs,
        };

        Ok(ride_detail)
    }

    pub fn get_ride_price(conn: &mut DbConnect, selected_id:i32) -> Result<i32, String> {
        rides.filter(id.eq(&selected_id))
            .select(price)
            .first(conn)
            .map_err(|e| e.to_string())
    }

    pub fn reassign_ride_staff(conn: &mut DbConnect, new_staff_id:i32, new_ride_id:i32) -> Result<i32, String> {
        reassign_staff_to_ride(conn, new_staff_id, new_ride_id)
    }

    pub fn check_ride_assignment_and_update(conn: &mut DbConnect, new_ride_id:i32) -> Result<(), String> {
        if !check_ride_staff_to_open(conn, new_ride_id)? {
            diesel::update(rides.filter(id.eq(new_ride_id)))
                .set(status.eq("Closed".to_string()))
                .execute(conn)
                .map_err(|e| format!("Error updating status: {}", e))?;
        }

        Ok(())
    }

    pub fn update_ride_status(conn:&mut DbConnect, ride_id:i32, new_status:String) -> Result<(), String>{
        if new_status == "Open" {
            if !check_ride_staff_to_open(conn, ride_id)? {
                return Err("Not enough staff assigned".to_string())
            }
        }

        diesel::update(rides.filter(id.eq(ride_id)))
            .set(status.eq(new_status))
            .execute(conn)
            .map_err(|e| format!("Error updating status: {}", e))?;

        Ok(())
    }

    pub fn create_ride(conn:&mut DbConnect, new_ride:NewRide) -> Result<(), String> {
        diesel::insert_into(rides)
            .values(new_ride)
            .execute(conn)
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn close_ride(conn: &mut DbConnect, ride_id:i32) -> Result<(), String> {
        diesel::update(rides).filter(id.eq(&ride_id))
            .set(status.eq("Shut Down".to_string()))
            .execute(conn)
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
use crate::DbConnect;
use crate::handler::image_handler::get_image_data;
use crate::models::{Ride, RideDetail};
use diesel::prelude::*;
use crate::schema::rides::dsl::rides;
use crate::schema::rides::id;

impl Ride {
    pub fn get_all_ride(conn:&mut DbConnect) -> Result<Vec<RideDetail>, String> {
        let other_rides = rides.select(Ride::as_select()).load(conn).map_err(|e| e.to_string())?;

        let ride_details: Vec<RideDetail> = other_rides
            .into_iter()
            .map(|ride| {
                let base64_image = get_image_data(conn, ride.image_id);

                RideDetail {
                    id:ride.id,
                    name:ride.name,
                    open_time:ride.open_time.to_string(),
                    close_time:ride.close_time.to_string(),
                    price:ride.price,
                    image_data:base64_image.unwrap(),
                    status:ride.status,
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
}
use crate::handler::image_handler::create_image;
use crate::{DbConnect, DbPool};
use bcrypt::{hash, DEFAULT_COST};
use chrono::NaiveTime;
use diesel::{QueryDsl, RunQueryDsl};
use mime_guess::from_path;
use std::fs;
use std::path::Path;
use crate::model::customer_model::NewCustomer;
use crate::model::lost_item_model::NewLostItem;
use crate::model::maintenance_report_model::NewMaintenanceReport;
use crate::model::menu_model::NewMenu;
use crate::model::notification_model::NewNotification;
use crate::model::ride_proposal_model::{NewRideProposal};
use crate::model::restaurant_model::NewRestaurant;
use crate::model::ride_model::NewRide;
use crate::model::souvenir_model::NewSouvenir;
use crate::model::staff_model::NewStaff;
use crate::model::store_model::NewStore;
use crate::model::store_proposal_model::NewStoreProposal;
use crate::schema::customers::dsl::customers;
use crate::schema::lost_items::dsl::lost_items;
use crate::schema::maintenance_reports::dsl::maintenance_reports;
use crate::schema::menus::dsl::menus;
use crate::schema::notifications::dsl::notifications;
use crate::schema::ride_proposals::dsl::ride_proposals;
use crate::schema::rides::dsl::rides;
use crate::schema::souvenirs::dsl::souvenirs;
use crate::schema::store_proposals::dsl::store_proposals;
use crate::schema::stores::dsl::stores;

pub fn seed_database(pool: &DbPool) {
    use crate::schema::restaurants::dsl::*;
    use crate::schema::staffs::dsl::*;
    let conn = &mut pool.get().unwrap();

    let staff_count: i64 = staffs.count().get_result(conn).unwrap();

    if staff_count == 0 {
        println!("Seeding");

        let staff_members = vec![
            NewStaff {
                name: "customerservice".to_string(),
                password: hash("customerservice", DEFAULT_COST)
                    .map_err(|_| "failed to hash".to_string())
                    .unwrap(),
                role: "Customer Service".to_string(),
            },
            NewStaff {
                name: "lostandfound".to_string(),
                password: hash("lostandfound", DEFAULT_COST)
                    .map_err(|_| "failed to hash".to_string())
                    .unwrap(),
                role: "Lost and Found Staff".to_string(),
            },
            NewStaff {
                name: "ridemanager".to_string(),
                password: hash("ridemanager", DEFAULT_COST)
                    .map_err(|_| "failed to hash".to_string())
                    .unwrap(),
                role: "Ride Manager".to_string(),
            },
            NewStaff {
                name: "ridestaff".to_string(),
                password: hash("ridestaff", DEFAULT_COST)
                    .map_err(|_| "failed to hash".to_string())
                    .unwrap(),
                role: "Ride Staff".to_string(),
            },
            NewStaff {
                name: "fbsupervisor".to_string(),
                password: hash("fbsupervisor", DEFAULT_COST)
                    .map_err(|_| "failed to hash".to_string())
                    .unwrap(),
                role: "F&B Supervisor".to_string(),
            },
            NewStaff {
                name: "chef".to_string(),
                password: hash("chef", DEFAULT_COST)
                    .map_err(|_| "failed to hash".to_string())
                    .unwrap(),
                role: "Chef".to_string(),
            },
            NewStaff {
                name: "waiter".to_string(),
                password: hash("waiter", DEFAULT_COST)
                    .map_err(|_| "failed to hash".to_string())
                    .unwrap(),
                role: "Waiter".to_string(),
            },
            NewStaff {
                name: "maintenancemanager".to_string(),
                password: hash("maintenancemanager", DEFAULT_COST)
                    .map_err(|_| "failed to hash".to_string())
                    .unwrap(),
                role: "Maintenance Manager".to_string(),
            },
            NewStaff {
                name: "maintenancestaff".to_string(),
                password: hash("maintenancestaff", DEFAULT_COST)
                    .map_err(|_| "failed to hash".to_string())
                    .unwrap(),
                role: "Maintenance Staff".to_string(),
            },
            NewStaff {
                name: "retailmanager".to_string(),
                password: hash("retailmanager", DEFAULT_COST)
                    .map_err(|_| "failed to hash".to_string())
                    .unwrap(),
                role: "Retail Manager".to_string(),
            },
            NewStaff {
                name: "salesassociate".to_string(),
                password: hash("salesassociate", DEFAULT_COST)
                    .map_err(|_| "failed to hash".to_string())
                    .unwrap(),
                role: "Sales Associate".to_string(),
            },
            NewStaff {
                name: "ceo".to_string(),
                password: hash("ceo", DEFAULT_COST)
                    .map_err(|_| "failed to hash".to_string())
                    .unwrap(),
                role: "CEO".to_string(),
            },
            NewStaff {
                name: "cfo".to_string(),
                password: hash("cfo", DEFAULT_COST)
                    .map_err(|_| "failed to hash".to_string())
                    .unwrap(),
                role: "CFO".to_string(),
            },
            NewStaff {
                name: "coo".to_string(),
                password: hash("coo", DEFAULT_COST)
                    .map_err(|_| "failed to hash".to_string())
                    .unwrap(),
                role: "COO".to_string(),
            },
        ];

        let seed_customers = vec![
            NewCustomer {
                name:"Willy".to_string(),
                balance:0
            },
            NewCustomer {
                name:"aldric".to_string(),
                balance:99999
            }
        ];

        let seed_restaurants = vec![
            NewRestaurant {
                name: "Kemuning".to_string(),
                image_id: seed_image(conn, "images/seed/kemuning.png").unwrap(),
                open_time: NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
                close_time: NaiveTime::from_hms_opt(18, 0, 0).unwrap(),
                cuisine: "Warteg Kehidupan".to_string(),
                status: "Closed".to_string(),
            },
            NewRestaurant {
                name: "Gyukaku".to_string(),
                image_id: seed_image(conn, "images/seed/gyukaku.png").unwrap(),
                open_time: NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
                close_time: NaiveTime::from_hms_opt(23, 0, 0).unwrap(),
                cuisine: "Japanese".to_string(),
                status: "Closed".to_string(),
            },
        ];

        let seed_stores = vec![
            NewStore {
                name:"Walmart".to_string(),
                image_id: seed_image(conn, "images/seed/walmart.png").unwrap(),
                open_time: NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
                close_time: NaiveTime::from_hms_opt(22, 0, 0).unwrap(),
                status: "Closed".to_string(),
            },
            NewStore {
                name:"Mito".to_string(),
                image_id: seed_image(conn, "images/seed/mito.png").unwrap(),
                open_time: NaiveTime::from_hms_opt(7, 0, 0).unwrap(),
                close_time: NaiveTime::from_hms_opt(20, 0, 0).unwrap(),
                status: "Closed".to_string(),
            },
        ];

        let seed_rides = vec![
            NewRide {
                image_id: seed_image(conn, "images/seed/ride1.png").unwrap(),
                name:"Rollercoaster Buatan Lord VK".to_string(),
                open_time: NaiveTime::from_hms_opt(9,0,0).unwrap(),
                close_time:NaiveTime::from_hms_opt(19,0,0).unwrap(),
                price:50000,
                status:"Closed".to_string()
            },
            NewRide {
                image_id: seed_image(conn, "images/seed/ride2.png").unwrap(),
                name:"Perosotan Kematian".to_string(),
                open_time: NaiveTime::from_hms_opt(8,0,0).unwrap(),
                close_time:NaiveTime::from_hms_opt(18,0,0).unwrap(),
                price:60000,
                status:"Pending Maintenance".to_string()
            },
            NewRide {
                image_id: seed_image(conn, "images/seed/ride3.png").unwrap(),
                name:"Rumah Maklo".to_string(),
                open_time: NaiveTime::from_hms_opt(10,0,0).unwrap(),
                close_time:NaiveTime::from_hms_opt(22,0,0).unwrap(),
                price:30000,
                status:"Pending Maintenance".to_string()
            },
        ];

        let seed_maintenance_report = vec![
            NewMaintenanceReport {
                ride_id: 2,
                description: "Ada orang patah kaki".to_string(),
                status:"Pending".to_string(),
            },
            NewMaintenanceReport {
                ride_id: 3,
                description: "Maklo Ilang".to_string(),
                status:"Pending".to_string(),
            }
        ];

        let seed_menus = vec![
            NewMenu {
                restaurant_id:1,
                image_id: seed_image(conn, "images/seed/telordadar.png").unwrap(),
                name:"Telor Dadar".to_string(),
                price:500000
            }
        ];

        let seed_souvenirs = vec![
            NewSouvenir{
                name:"Stuffed Dog".to_string(),
                store_id:1,
                image_id: seed_image(conn, "images/seed/telordadar.png").unwrap(),
                price:300000,
                description:"A stuffed dog".to_string()
            },
            NewSouvenir{
                name:"Egg Stuffed Doll".to_string(),
                store_id:1,
                image_id: seed_image(conn, "images/seed/telordadar.png").unwrap(),
                price:50,
                description:"An Egg".to_string()
            }
        ];

        let seed_lost_item = vec![
            NewLostItem {
                name:"Chill Guy".to_string(),
                item_type:"Toy".to_string(),
                color:"Brown".to_string(),
                last_location:"Rumah Maklo".to_string(),
                owner_id:1,
                status:"Found".to_string(),
                finder_id:Some(2),
                found_location:Some("Perosotan Kematian".to_string()),
                image_id:Some(seed_image(conn, "images/seed/chillguy.png").unwrap())
            },
            NewLostItem {
                name:"Handphone samsung".to_string(),
                item_type:"Gadget".to_string(),
                color:"White".to_string(),
                last_location:"Walmart".to_string(),
                owner_id:2,
                status:"Missing".to_string(),
                finder_id:None,
                found_location:None,
                image_id:None
            }
        ];

        let seed_notifications = vec![
            NewNotification {
                customer_id:1,
                message:"An item has been Found".to_string()
            }
        ];

        let seed_ride_proposals = vec![
            NewRideProposal {
                proposal_type:"New".to_string(),
                status:"Pending".to_string(),
                description:"Cinema Ride".to_string(),
                price:50000,
                ride_id:None,
                image_id:Some(seed_image(conn, "images/seed/tommy.jpg").unwrap())
            },
            NewRideProposal {
                proposal_type:"Remove".to_string(),
                status:"Pending".to_string(),
                description:"Jele".to_string(),
                price:30000,
                ride_id:Some(1),
                image_id:None,
            },
        ];

        let seed_store_proposals = vec![
            NewStoreProposal {
                name:"Toko Teh Obeng".to_string(),
                proposal_type:"New".to_string(),
                status:"Pending".to_string(),
                description:"Toko Teh Obeng".to_string(),
                store_id:None,
                image_id:Some(seed_image(conn, "images/seed/tommy.jpg").unwrap())
            },
            NewStoreProposal {
                name:"Perosotan Kematian".to_string(),
                proposal_type:"Remove".to_string(),
                status:"Pending".to_string(),
                description:"Jele".to_string(),
                store_id:Some(1),
                image_id:None,
            },
        ];


        diesel::insert_into(staffs)
            .values(&staff_members)
            .execute(conn)
            .map_err(|e| e.to_string())
            .unwrap();

        diesel::insert_into(customers)
            .values(&seed_customers)
            .execute(conn)
            .map_err(|e| e.to_string())
            .unwrap();

        diesel::insert_into(restaurants)
            .values(&seed_restaurants)
            .execute(conn)
            .map_err(|e| e.to_string())
            .unwrap();

        diesel::insert_into(stores)
            .values(&seed_stores)
            .execute(conn)
            .map_err(|e| e.to_string())
            .unwrap();

        diesel::insert_into(menus)
            .values(&seed_menus)
            .execute(conn)
            .map_err(|e| e.to_string())
            .unwrap();

        diesel::insert_into(souvenirs)
            .values(&seed_souvenirs)
            .execute(conn)
            .map_err(|e| e.to_string())
            .unwrap();

        diesel::insert_into(rides)
            .values(&seed_rides)
            .execute(conn)
            .map_err(|e| e.to_string())
            .unwrap();

        diesel::insert_into(maintenance_reports)
            .values(&seed_maintenance_report)
            .execute(conn)
            .map_err(|e| e.to_string())
            .unwrap();

        diesel::insert_into(lost_items)
            .values(&seed_lost_item)
            .execute(conn)
            .map_err(|e| e.to_string())
            .unwrap();

        diesel::insert_into(notifications)
            .values(&seed_notifications)
            .execute(conn)
            .map_err(|e| e.to_string())
            .unwrap();

        diesel::insert_into(ride_proposals)
            .values(&seed_ride_proposals)
            .execute(conn)
            .map_err(|e| e.to_string())
            .unwrap();

        diesel::insert_into(store_proposals)
            .values(&seed_store_proposals)
            .execute(conn)
            .map_err(|e| e.to_string())
            .unwrap();

    }
}

pub fn seed_image(conn: &mut DbConnect, file_path: &str) -> Result<i32, String> {
    let path = Path::new(file_path);
    let image_data = fs::read(&path).map_err(|e| e.to_string())?;

    let mime_type = from_path(&path)
        .first()
        .map(|m| m.to_string())
        .unwrap_or_else(|| "application/octet-stream".to_string());

    let filename = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or("Invalid filename".to_string())?
        .to_string();

    create_image(conn, image_data, mime_type, filename)
}

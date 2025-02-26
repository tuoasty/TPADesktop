use bcrypt::{hash, DEFAULT_COST};
use diesel::{QueryDsl, RunQueryDsl};
use crate::{DbPool};
use crate::models::NewStaff;

pub fn seed_database(pool: &DbPool) {
    use crate::schema::staffs::dsl::*;
    let conn = &mut pool.get().unwrap();

    let staff_count: i64 = staffs.count().get_result(conn).unwrap();

    if staff_count == 0 {
        println!("Seeding");

        let staff_members = vec![
            NewStaff {
                name:"customerservice".to_string(),
                password:hash("customerservice", DEFAULT_COST).map_err(|_| "failed to hash".to_string()).unwrap(),
                role:"Customer Service".to_string()
            },
            NewStaff {
                name:"lostandfound".to_string(),
                password:hash("lostandfound", DEFAULT_COST).map_err(|_| "failed to hash".to_string()).unwrap(),
                role:"Lost and Found Staff".to_string()
            },
            NewStaff {
                name:"ridemanager".to_string(),
                password:hash("ridemanager", DEFAULT_COST).map_err(|_| "failed to hash".to_string()).unwrap(),
                role:"Ride Manager".to_string()
            },
            NewStaff {
                name:"ridestaff".to_string(),
                password:hash("ridestaff", DEFAULT_COST).map_err(|_| "failed to hash".to_string()).unwrap(),
                role:"Ride Staff".to_string()
            },
            NewStaff {
                name:"fbsupervisor".to_string(),
                password:hash("fbsupervisor", DEFAULT_COST).map_err(|_| "failed to hash".to_string()).unwrap(),
                role:"F&B Supervisor".to_string()
            },
            NewStaff {
                name:"chef".to_string(),
                password:hash("chef", DEFAULT_COST).map_err(|_| "failed to hash".to_string()).unwrap(),
                role:"Chef".to_string()
            },
            NewStaff {
                name:"waiter".to_string(),
                password:hash("waiter", DEFAULT_COST).map_err(|_| "failed to hash".to_string()).unwrap(),
                role:"Waiter".to_string()
            },
            NewStaff {
                name:"maintenancemanager".to_string(),
                password:hash("maintenancemanager", DEFAULT_COST).map_err(|_| "failed to hash".to_string()).unwrap(),
                role:"Maintenance Manager".to_string()
            },
            NewStaff {
                name:"maintenancestaff".to_string(),
                password:hash("maintenancestaff", DEFAULT_COST).map_err(|_| "failed to hash".to_string()).unwrap(),
                role:"Maintenance Staff".to_string()
            },
            NewStaff {
                name:"retailmanager".to_string(),
                password:hash("retailmanager", DEFAULT_COST).map_err(|_| "failed to hash".to_string()).unwrap(),
                role:"Retail Manager".to_string()
            },
            NewStaff {
                name:"salesassociate".to_string(),
                password:hash("salesassociate", DEFAULT_COST).map_err(|_| "failed to hash".to_string()).unwrap(),
                role:"Sales Associate".to_string()
            },
            NewStaff {
                name:"ceo".to_string(),
                password:hash("ceo", DEFAULT_COST).map_err(|_| "failed to hash".to_string()).unwrap(),
                role:"CEO".to_string()
            },
            NewStaff {
                name:"cfo".to_string(),
                password:hash("cfo", DEFAULT_COST).map_err(|_| "failed to hash".to_string()).unwrap(),
                role:"CFO".to_string()
            },
            NewStaff {
                name:"coo".to_string(),
                password:hash("coo", DEFAULT_COST).map_err(|_| "failed to hash".to_string()).unwrap(),
                role:"COO".to_string()
            },
        ];

        diesel::insert_into(staffs)
            .values(&staff_members)
            .execute(conn)
            .map_err(|e| e.to_string()).unwrap();
    }
}
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
        let coo_password = hash("COO", DEFAULT_COST).map_err(|_| "failed to hash".to_string()).unwrap();
        let coo = NewStaff {
            name:"COO".to_string(),
            password:coo_password,
            admin:true,
        };

        diesel::insert_into(staffs)
            .values(coo)
            .execute(conn)
            .map_err(|e| e.to_string()).unwrap();
    }
}
use crate::models::Staff;
use crate::schema::staffs::dsl::staffs;
use crate::schema::staffs::name;
use crate::DbConnect;
use diesel::prelude::*;

impl Staff {
    pub fn get_staff(conn: &mut DbConnect, username: &str) -> Result<Self, String> {
        staffs
            .filter(name.eq(&username))
            .first(conn)
            .map_err(|e| e.to_string())
    }
}

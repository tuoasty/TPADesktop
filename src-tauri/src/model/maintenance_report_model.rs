use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize)]
#[diesel(belongs_to(Ride, foreign_key = ride_id))]
#[diesel(belongs_to(Staff, foreign_key = staff_id))]
#[diesel(table_name = crate::schema::maintenance_reports)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MaintenanceReport {
    pub id: i32,
    pub ride_id: i32,
    pub description: String,
    pub status: String
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::maintenance_reports)]
pub struct NewMaintenanceReport {
    pub ride_id: i32,
    pub description: String,
    pub status: String
}

#[derive(Serialize)]
pub struct MaintenanceReportDetail {
    pub id: i32,
    pub ride_id: i32,
    pub ride_name: String,
    pub staff_id: Option<i32>,
    pub staff_name: Option<String>,
    pub description: String,
    pub status: String
}
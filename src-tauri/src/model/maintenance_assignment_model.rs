use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable)]
#[diesel(belongs_to(MaintenanceReport, foreign_key = maintenance_report_id))]
#[diesel(belongs_to(Staff, foreign_key = staff_id))]
#[diesel(table_name = crate::schema::maintenance_assignments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MaintenanceAssignment {
    pub id: i32,
    pub staff_id: i32,
    pub maintenance_report_id:i32,
    pub assignment_date:NaiveDateTime,
    pub status:String
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::maintenance_assignments)]
pub struct NewMaintenanceAssignment {
    pub staff_id: i32,
    pub maintenance_report_id:i32,
    pub assignment_date:NaiveDateTime,
    pub status:String
}
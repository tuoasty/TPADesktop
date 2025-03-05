use diesel::{Insertable, Queryable, Selectable};
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(belongs_to(Customer, foreign_key = customer_id))]
#[diesel(belongs_to(Souvenir, foreign_key = souvenir_id))]
#[diesel(belongs_to(Store, foreign_key = store_id))]
#[diesel(table_name = crate::schema::store_transactions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StoreTransaction {
    pub id:i32,
    pub customer_id:i32,
    pub store_id:i32,
    pub souvenir_id:i32,
    pub count:i32
}

#[derive(Insertable, Serialize)]
#[diesel(table_name = crate::schema::store_transactions)]
pub struct NewStoreTransaction {
    pub customer_id:i32,
    pub store_id:i32,
    pub souvenir_id:i32,
    pub  count:i32
}
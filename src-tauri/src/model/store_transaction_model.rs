use chrono::NaiveTime;
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable)]
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
    pub count:i32,
    pub value:i32,
    pub time_ordered:NaiveTime
}

#[derive(Deserialize, Serialize)]
pub struct StoreTransactionDetail {
    pub customer_id:i32,
    pub store_id:i32,
    pub souvenir_id:i32,
    pub souvenir_name:String,
    pub count:i32,
    pub value:i32
}
#[derive(Insertable)]
#[diesel(table_name = crate::schema::store_transactions)]
pub struct NewStoreTransaction {
    pub customer_id:i32,
    pub store_id:i32,
    pub souvenir_id:i32,
    pub count:i32,
    pub value:i32,
    pub time_ordered:NaiveTime
}

#[derive(Serialize)]
pub struct StoreRevenue {
    pub id:i32,
    pub time:String,
    pub value:i32,
    pub store_id:i32,
    pub souvenir_id:i32
}
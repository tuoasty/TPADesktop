use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use dotenvy::dotenv;
use r2d2::{Pool, PooledConnection};
use std::env;
use std::sync::Mutex;
use tauri::command;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod handler;
mod repo;
pub mod schema;
pub mod seed;
mod model;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnect = r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;
pub struct CurrentStaff(pub Mutex<Option<(i32, String, String)>>);

pub struct CurrentCustomer(pub Mutex<Option<(i32, String, i32)>>);

use redis::{Client, Commands, RedisError};
use serde_json;

pub struct RedisCache {
    client: Client,
}

impl RedisCache {
    pub fn new(redis_url: &str) -> Result<Self, RedisError> {
        let client = Client::open(redis_url)?;
        Ok(Self { client })
    }

    pub fn get_ride(&self, id: i32) -> Option<model::ride_model::RideDetail> {
        let mut conn = match self.client.get_connection() {
            Ok(conn) => conn,
            Err(_) => return None,
        };

        let key = format!("ride:{}", id);
        let data: Option<String> = match conn.get(&key) {
            Ok(data) => data,
            Err(_) => return None,
        };

        if let Some(json_data) = data {
            return match serde_json::from_str(&json_data) {
                Ok(ride) => Some(ride),
                Err(_) => None,
            };
        }

        None
    }

    pub fn set_ride(&self, ride: &model::ride_model::RideDetail) -> Result<(), RedisError> {
        let mut conn = self.client.get_connection()?;
        let key = format!("ride:{}", ride.id);

        let json_data = serde_json::to_string(ride).unwrap();
        conn.set_ex(key, json_data, 1800)?;
        Ok(())
    }

    pub fn invalidate_ride(&self, id: i32) -> Result<(), RedisError> {
        let mut conn = self.client.get_connection()?;
        let key = format!("ride:{}", id);
        conn.del(key)?;
        Ok(())
    }
}

fn establish_connection() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .max_size(10)
        .build(manager)
        .expect("Failed to create database")
}

pub fn get_conn(
    pool: &DbPool,
) -> Result<PooledConnection<ConnectionManager<PgConnection>>, String> {
    pool.get().map_err(|_| "failed to get DB pool".to_string())
}

#[command]
fn get_app_id() -> Result<String, String> {
    let path = std::path::Path::new("./config.conf");
    let content = match std::fs::read_to_string(path) {
        Ok(file) => file,
        Err(e) => return Err(format!("Error : {}", e)),
    };

    let id = content
        .lines()
        .find_map(|line| {
            if line.starts_with("AppId=") {
                Some(line.trim_start_matches("AppId=").to_string())
            } else {
                None
            }
        })
        .unwrap_or_else(|| "2".to_string());
    Ok(id)
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[tokio::main]
pub async fn run() {
    let pool = establish_connection();
    let current_staff = CurrentStaff(Mutex::new(None));
    let current_customer = CurrentCustomer(Mutex::new(None));

    dotenv().ok();
    dotenv().ok();
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://default:6Pe97ZWcNwITTlxkkzhLGA2cVEsEOYzj@redis-16407.c334.asia-southeast2-1.gce.redns.redis-cloud.com:16407".to_string());
    let redis_cache = RedisCache::new(&redis_url).expect("Failed to connect to Redis");

    seed::seed_database(&pool);

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(pool)
        .manage(current_staff)
        .manage(current_customer)
        .manage(redis_cache)
        .invoke_handler(all_handlers!())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

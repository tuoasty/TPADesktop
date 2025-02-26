use std::env;
use std::sync::Mutex;
use diesel::PgConnection;
use r2d2::{Pool, PooledConnection};
use diesel::r2d2::{self, ConnectionManager};
use dotenvy::dotenv;
use tauri::command;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod models;
pub mod schema;
pub mod seed;
mod handler;
mod model;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnect = r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

pub struct CurrentStaff(pub Mutex<Option<(i32, String, String)>>);
fn establish_connection() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().max_size(10).build(manager).expect("Failed to create database")
}

pub fn get_conn(pool:&DbPool) -> Result<PooledConnection<ConnectionManager<PgConnection>>, String> {
    pool.get().map_err(|_| "failed to get DB pool".to_string())
}

#[command]
fn get_app_id() -> Result<String, String> {
    let path = std::path::Path::new("./config.conf");
    let content = match std::fs::read_to_string(path) {
        Ok(file) => file,
        Err(e) => return Err(format!("Error : {}", e)),
    };

    let id = content.lines().find_map(|line| {
        if line.starts_with("AppId="){
            Some(line.trim_start_matches("AppId=").to_string())
        } else {
            None
        }
    }).unwrap_or_else(|| "customer".to_string());
    eprintln!("{}", id);
    Ok(id)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let pool = establish_connection();
    let current_staff = CurrentStaff(Mutex::new(None));

    seed::seed_database(&pool);

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(pool)
        .manage(current_staff)
        .invoke_handler(tauri::generate_handler![get_app_id])
        .invoke_handler(all_handlers!())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

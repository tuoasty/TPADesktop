use std::env;
use std::sync::Mutex;
use diesel::PgConnection;
use r2d2::Pool;
use diesel::r2d2::{self, ConnectionManager};
use dotenvy::dotenv;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod db;
pub mod models;
pub mod schema;
type DbPool = Pool<ConnectionManager<PgConnection>>;

pub struct CurrentUser(pub Mutex<Option<(i32, String)>>);
fn establish_connection() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().max_size(10).build(manager).expect("Failed to create database")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let pool = establish_connection();
    let current_user = CurrentUser(Mutex::new(None));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(pool)
        .manage(current_user)
        .invoke_handler(tauri::generate_handler![db::login_user, db::register_user, db::get_user])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

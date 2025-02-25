pub mod db;
pub mod staff_handler;

#[macro_export]
macro_rules! all_commands {
    () => {
        tauri::generate_handler![
            $crate::handlers::db::register_staff,
            $crate::handlers::staff_handler::get_staff,
            $crate::handlers::staff_handler::login_staff,
            $crate::handlers::staff_handler::logout_staff,
            // Add all other commands
        ]
    };
}
pub mod staff_handler;
pub mod staff;

#[macro_export]
macro_rules! all_commands {
    () => {
        tauri::generate_handler![
            $crate::handlers::staff_handler::register_staff,
            $crate::handlers::staff::get_staff,
            $crate::handlers::staff::login_staff,
            $crate::handlers::staff::logout_staff,
            // Add all other commands
        ]
    };
}
pub mod staff_handler;
pub mod image_handler;

#[macro_export]
macro_rules! all_handlers {
    () => {
        tauri::generate_handler![
            $crate::handler::staff_handler::register_staff,
            // Add all other commands
        ]
    };
}
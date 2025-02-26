pub mod staff_handler;
pub mod image_handler;
pub mod restaurant_handler;

#[macro_export]
macro_rules! all_handlers {
    () => {
        tauri::generate_handler![
            $crate::handler::staff_handler::create_staff,
            $crate::handler::staff_handler::login_staff,
            $crate::handler::staff_handler::logout_staff,
            $crate::handler::staff_handler::get_current_staff,
            $crate::handler::restaurant_handler::find_restaurant,
            $crate::handler::restaurant_handler::find_all_restaurant,
            // Add all other commands
        ]
    };
}
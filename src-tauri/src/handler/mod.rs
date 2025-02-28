pub mod image_handler;
pub mod restaurant_handler;
pub mod staff_handler;
pub mod store_handler;
pub mod menu_handler;
pub mod souvenir_handler;
pub mod ride_handler;

#[macro_export]
macro_rules! all_handlers {
    () => {
        tauri::generate_handler![
            $crate::handler::staff_handler::create_staff,
            $crate::handler::staff_handler::verify_authentication,
            $crate::handler::staff_handler::verify_login,
            $crate::handler::staff_handler::login_staff,
            $crate::handler::staff_handler::logout_staff,
            $crate::handler::staff_handler::get_current_staff,
            $crate::handler::restaurant_handler::find_all_restaurant,
            $crate::handler::menu_handler::create_menu,
            $crate::handler::store_handler::find_all_store,
            $crate::handler::souvenir_handler::remove_souvenir,
            $crate::handler::ride_handler::find_all_ride,
            // Add all other commands
        ]
    };
}

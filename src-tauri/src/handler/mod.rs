pub mod image_handler;
pub mod restaurant_handler;
pub mod staff_handler;
pub mod store_handler;
pub mod menu_handler;
pub mod souvenir_handler;
pub mod ride_handler;
pub mod maintenance_report_handler;
pub mod maintenance_assignment_handler;
pub mod restaurant_assignment_handler;
pub mod store_assignment_handler;
pub mod ride_assignment_handler;

#[macro_export]
macro_rules! all_handlers {
    () => {
        tauri::generate_handler![
            get_app_id,
            $crate::handler::staff_handler::create_staff,
            $crate::handler::staff_handler::verify_authentication,
            $crate::handler::staff_handler::verify_login,
            $crate::handler::staff_handler::login_staff,
            $crate::handler::staff_handler::logout_staff,
            $crate::handler::staff_handler::get_current_staff,
            $crate::handler::staff_handler::find_all_staff,
            $crate::handler::restaurant_handler::find_all_restaurant,
            $crate::handler::restaurant_handler::change_restaurant_status,
            $crate::handler::restaurant_handler::find_all_consumption_staff,
            $crate::handler::restaurant_handler::reassign_restaurant_and_check_status,
            $crate::handler::menu_handler::create_menu,
            $crate::handler::store_handler::find_all_store,
            $crate::handler::store_handler::change_store_status,
            $crate::handler::store_handler::reassign_store_and_check_status,
            $crate::handler::souvenir_handler::remove_souvenir,
            $crate::handler::ride_handler::find_all_ride,
            $crate::handler::ride_handler::change_ride_status,
            $crate::handler::ride_handler::reassign_ride_and_check_status,
            $crate::handler::maintenance_report_handler::find_all_maintenance_report,
            $crate::handler::maintenance_report_handler::find_all_maintenance_staff,
            $crate::handler::maintenance_report_handler::accept_request,
            $crate::handler::maintenance_report_handler::reject_request,
            $crate::handler::restaurant_assignment_handler::assign_staff_to_restaurant,
            $crate::handler::store_assignment_handler::assign_staff_to_store,
            $crate::handler::ride_assignment_handler::assign_staff_to_ride,
            // Add all other commands
        ]
    };
}

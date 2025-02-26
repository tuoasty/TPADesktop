pub mod staff;

#[macro_export]
macro_rules! all_models {
    () => {
        tauri::generate_handler![
            $crate::model::staff::get_staff,
            $crate::model::staff::login_staff,
            $crate::model::staff::logout_staff,
            // Add all other commands
        ]
    };
}
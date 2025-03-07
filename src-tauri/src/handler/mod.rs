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
pub mod lost_item_handler;
pub mod customer_handler;
pub mod notification_handler;
pub mod ride_proposal_handler;
pub mod store_proposal_handler;
pub mod restaurant_proposal_handler;
pub mod store_transaction_handler;
pub mod chat_handler;
pub mod ride_queue_handler;
pub mod restaurant_order_handler;

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
            $crate::handler::restaurant_handler::find_restaurant_by_id,
            $crate::handler::restaurant_handler::find_staff_restaurant,
            $crate::handler::menu_handler::create_menu,
            $crate::handler::store_handler::find_all_store,
            $crate::handler::store_handler::change_store_status,
            $crate::handler::store_handler::reassign_store_and_check_status,
            $crate::handler::store_handler::find_store_by_id,
            $crate::handler::store_handler::find_staff_store,
            $crate::handler::store_handler::find_store_transaction,
            $crate::handler::souvenir_handler::remove_souvenir,
            $crate::handler::souvenir_handler::purchase_souvenir,
            $crate::handler::ride_handler::find_all_ride,
            $crate::handler::ride_handler::change_ride_status,
            $crate::handler::ride_handler::reassign_ride_and_check_status,
            $crate::handler::ride_handler::find_ride_by_id,
            $crate::handler::ride_handler::find_staff_ride,
            $crate::handler::ride_handler::report_ride_maintenance,
            $crate::handler::maintenance_report_handler::find_all_maintenance_report,
            $crate::handler::maintenance_report_handler::find_all_maintenance_staff,
            $crate::handler::maintenance_report_handler::accept_request,
            $crate::handler::maintenance_report_handler::reject_request,
            $crate::handler::maintenance_report_handler::find_staff_maintenance,
            $crate::handler::maintenance_report_handler::submit_task,
            $crate::handler::restaurant_assignment_handler::assign_staff_to_restaurant,
            $crate::handler::store_assignment_handler::assign_staff_to_store,
            $crate::handler::ride_assignment_handler::assign_staff_to_ride,
            $crate::handler::lost_item_handler::find_all_lost_item,
            $crate::handler::lost_item_handler::update_lost_item,
            $crate::handler::customer_handler::verify_customer_login,
            $crate::handler::customer_handler::login_customer,
            $crate::handler::customer_handler::logout_customer,
            $crate::handler::customer_handler::get_current_customer,
            $crate::handler::customer_handler::create_customer_account,
            $crate::handler::customer_handler::get_all_customer,
            $crate::handler::notification_handler::find_customer_notifications,
            $crate::handler::notification_handler::broadcast_message,
            $crate::handler::ride_proposal_handler::find_ride_proposal,
            $crate::handler::ride_proposal_handler::accept_ride_proposal,
            $crate::handler::ride_proposal_handler::reject_ride_proposal,
            $crate::handler::ride_proposal_handler::propose_new_ride,
            $crate::handler::store_proposal_handler::find_store_proposal,
            $crate::handler::store_proposal_handler::accept_store_proposal,
            $crate::handler::store_proposal_handler::reject_store_proposal,
            $crate::handler::restaurant_proposal_handler::propose_new_restaurant,
            $crate::handler::restaurant_proposal_handler::find_restaurant_proposals,
            $crate::handler::restaurant_proposal_handler::accept_restaurant_proposal,
            $crate::handler::restaurant_proposal_handler::reject_restaurant_proposal,
            $crate::handler::chat_handler::send_chat_message,
            $crate::handler::chat_handler::fetch_new_messages,
            $crate::handler::ride_queue_handler::find_ride_queue,
            $crate::handler::ride_queue_handler::add_customer_to_ride_queue,
            $crate::handler::ride_queue_handler::dequeue_customer_from_ride,
            $crate::handler::ride_queue_handler::find_ride_revenue,
            $crate::handler::restaurant_order_handler::order_restaurant_food,
            $crate::handler::restaurant_order_handler::find_restaurant_orders,
            $crate::handler::restaurant_order_handler::set_order_status,
            $crate::handler::restaurant_order_handler::find_restaurant_revenue,
            $crate::handler::store_transaction_handler::find_store_revenue,
        ]
    };
}

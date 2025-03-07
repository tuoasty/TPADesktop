use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use tauri::{command, State};
use crate::{get_conn, DbPool};
use crate::model::restaurant_proposal_model::{NewRestaurantProposalDetail, RestaurantProposal, RestaurantProposalDetail};

#[command]
pub fn propose_new_restaurant(state:State<DbPool>, new_proposal:NewRestaurantProposalDetail) -> Result<(), String> {
    if new_proposal.cuisine.is_empty() || new_proposal.image_data.is_empty() || new_proposal.mime_type.is_empty() || new_proposal.image_name.is_empty() {
        return Err("All fields must be filled".to_string())
    };

    let conn = &mut get_conn(&state)?;

    let image_data = STANDARD.decode(&new_proposal.image_data).map_err(|_| "Invalid Base encoding".to_string())?;

    RestaurantProposal::propose_new_restaurant(conn, new_proposal, image_data)
}

#[command]
pub fn find_restaurant_proposals(state:State<DbPool>) -> Result<Vec<RestaurantProposalDetail>, String> {
    let conn = &mut get_conn(&state)?;

    RestaurantProposal::get_restaurant_proposals(conn)
}

#[command]
pub fn accept_restaurant_proposal(state:State<DbPool>, proposal_id:i32, role:String, current_status:String) -> Result<(), String> {
    let conn = &mut get_conn(&state)?;

    let mut new_status = "Pending".to_string();

    if current_status == "Accepted by CEO" && role == "CFO" {
        new_status = "Accepted".to_string()
    } else if current_status == "Accepted by CFO" && role == "CEO" {
        new_status = "Accepted".to_string()
    } else if role == "CEO" {
        new_status = "Accepted by CEO".to_string()
    } else if role == "CFO" {
        new_status = "Accepted by CFO".to_string()
    } else {
        new_status = current_status
    }

    RestaurantProposal::accept_restaurant_proposal(conn, proposal_id, new_status)
}
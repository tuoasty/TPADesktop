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
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use tauri::{command, State};
use crate::{get_conn, DbPool};
use crate::model::ride_proposal_model::{NewRideProposalDetail, RideProposal, RideProposalDetail};

#[command]
pub fn propose_new_ride(state:State<DbPool>, new_proposal:NewRideProposalDetail) -> Result<(), String> {
    if new_proposal.name.is_empty() || new_proposal.image_data.is_empty() ||
        new_proposal.image_name.is_empty() || new_proposal.mime_type.is_empty() ||
        new_proposal.description.is_empty() {
        return Err("All fields must be filled".to_string())
    };

    if new_proposal.price <= 0 {
        return Err("Price must be more than 0".to_string())
    };

    let conn = &mut get_conn(&state)?;
    let image_data = STANDARD.decode(&new_proposal.image_data).map_err(|_| "Invalid Base encoding".to_string())?;
    RideProposal::propose_new_ride(conn, new_proposal, image_data)
}
#[command]
pub fn find_ride_proposal(state:State<DbPool>) -> Result<Vec<RideProposalDetail>, String> {
    let conn = &mut get_conn(&state)?;

    RideProposal::get_ride_proposals(conn)
}

#[command]
pub fn accept_ride_proposal(state:State<DbPool>, proposal_id:i32) -> Result<(), String> {
    let conn = &mut get_conn(&state)?;

    RideProposal::accept_ride_proposal(conn, proposal_id)
}

#[command]
pub fn reject_ride_proposal(state:State<DbPool>, proposal_id:i32) -> Result<(), String> {
    let conn = &mut get_conn(&state)?;

    RideProposal::reject_ride_proposal(conn, proposal_id)
}
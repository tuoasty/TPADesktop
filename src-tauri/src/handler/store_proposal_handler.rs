use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use tauri::{command, State};
use crate::{get_conn, DbPool};
use crate::model::store_proposal_model::{NewStoreProposalDetail, StoreProposal, StoreProposalDetail};

#[command]
pub fn propose_new_store(state:State<DbPool>, new_proposal:NewStoreProposalDetail) -> Result<(), String> {
    if new_proposal.name.is_empty() || new_proposal.image_data.is_empty() ||
        new_proposal.image_name.is_empty() || new_proposal.mime_type.is_empty() ||
        new_proposal.description.is_empty() {
        return Err("All fields must be filled".to_string())
    };

    let conn = &mut get_conn(&state)?;
    let image_data = STANDARD.decode(&new_proposal.image_data).map_err(|_| "Invalid Base encoding".to_string())?;
    StoreProposal::propose_new_store(conn, new_proposal, image_data)
}
#[command]
pub fn find_store_proposal(state:State<DbPool>) -> Result<Vec<StoreProposalDetail>, String> {
    let conn = &mut get_conn(&state)?;

    StoreProposal::get_store_proposals(conn)
}

#[command]
pub fn accept_store_proposal(state:State<DbPool>, proposal_id:i32) -> Result<(), String> {
    let conn = &mut get_conn(&state)?;

    StoreProposal::accept_store_proposal(conn, proposal_id)
}

#[command]
pub fn reject_store_proposal(state:State<DbPool>, proposal_id:i32) -> Result<(), String> {
    let conn = &mut get_conn(&state)?;

    StoreProposal::reject_store_proposal(conn, proposal_id)
}
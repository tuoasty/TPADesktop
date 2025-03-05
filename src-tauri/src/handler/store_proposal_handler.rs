use tauri::{command, State};
use crate::{get_conn, DbPool};
use crate::model::store_proposal_model::{StoreProposal, StoreProposalDetail};

#[command]
pub fn find_store_proposal(state:State<DbPool>) -> Result<Vec<StoreProposalDetail>, String> {
    let conn = &mut get_conn(&state)?;

    StoreProposal::get_store_proposals(conn)
}

#[command]
pub fn accept_store_proposal(state:State<DbPool>, proposal_id:i32) -> Result<(), String> {
    let conn = &mut get_conn(&state)?;

    StoreProposal::accept_proposal(conn, proposal_id)
}

#[command]
pub fn reject_store_proposal(state:State<DbPool>, proposal_id:i32) -> Result<(), String> {
    let conn = &mut get_conn(&state)?;

    StoreProposal::reject_proposal(conn, proposal_id)
}
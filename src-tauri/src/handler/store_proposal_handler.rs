use tauri::{command, State};
use crate::{get_conn, DbPool};
use crate::model::store_proposal_model::{StoreProposal, StoreProposalDetail};

#[command]
pub fn find_store_proposal(state:State<DbPool>) -> Result<Vec<StoreProposalDetail>, String> {
    let conn = &mut get_conn(&state)?;

    StoreProposal::get_store_proposals(conn)
}
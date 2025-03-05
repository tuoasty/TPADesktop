use tauri::{command, State};
use crate::{get_conn, DbPool};
use crate::model::ride_proposal_model::{RideProposal, RideProposalDetail};

#[command]
pub fn find_ride_proposal(state:State<DbPool>) -> Result<Vec<RideProposalDetail>, String> {
    let conn = &mut get_conn(&state)?;

    RideProposal::get_ride_proposals(conn)
}
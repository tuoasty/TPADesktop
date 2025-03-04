use tauri::{command, State};
use crate::{get_conn, DbPool};
use crate::model::ride_proposal_model::RideProposal;

#[command]
pub fn find_ride_proposal(state:State<DbPool>) -> Result<Vec<RideProposal>, String> {
    let conn = &mut get_conn(&state)?;

    RideProposal::get_ride_proposals(conn)
}
use crate::DbConnect;
use crate::model::ride_proposal_model::{RideProposal};
use diesel::prelude::*;
use crate::schema::ride_proposals::dsl::ride_proposals;

impl RideProposal {
    pub fn get_ride_proposals(conn: &mut DbConnect) -> Result<Vec<Self>, String> {
        ride_proposals
            .select(RideProposal::as_select())
            .load(conn)
            .map_err(|e| e.to_string())
    }
}
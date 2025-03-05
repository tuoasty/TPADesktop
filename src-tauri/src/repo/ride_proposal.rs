use crate::DbConnect;
use crate::model::ride_proposal_model::{RideProposal, RideProposalDetail};
use diesel::prelude::*;
use crate::handler::image_handler::{get_image_data};
use crate::handler::ride_handler::find_ride;
use crate::schema::ride_proposals::dsl::ride_proposals;

impl RideProposal {
    pub fn get_ride_proposals(conn: &mut DbConnect) -> Result<Vec<RideProposalDetail>, String> {
        let proposals = ride_proposals
            .select(RideProposal::as_select())
            .load(conn)
            .map_err(|e| e.to_string())?;

        let proposal_details: Vec<RideProposalDetail> = proposals
            .into_iter()
            .map(|proposal| {

                let image_data = proposal.image_id
                    .and_then(|image| get_image_data(conn, image).ok());

                let ride_name = proposal.ride_id
                    .and_then(|id| find_ride(conn, id).ok().map(|ride| ride.name));

                RideProposalDetail {
                    id:proposal.id,
                    proposal_type:proposal.proposal_type,
                    status:proposal.status,
                    description:proposal.description,
                    ride_id:proposal.ride_id,
                    ride_name,
                    image_data
                }
            })
            .collect();

        Ok(proposal_details)
    }
}
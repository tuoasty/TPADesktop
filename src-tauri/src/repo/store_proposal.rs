use crate::DbConnect;
use crate::handler::image_handler::get_image_data;
use crate::model::store_proposal_model::{StoreProposal, StoreProposalDetail};
use diesel::prelude::*;
use crate::handler::store_handler::find_store;
use crate::schema::store_proposals::dsl::store_proposals;

impl StoreProposal {
    pub fn get_store_proposals(conn: &mut DbConnect) -> Result<Vec<StoreProposalDetail>, String> {
        let proposals = store_proposals
            .select(StoreProposal::as_select())
            .load(conn)
            .map_err(|e| e.to_string())?;

        let proposal_details: Vec<StoreProposalDetail> = proposals
            .into_iter()
            .map(|proposal| {

                let image_data = proposal.image_id
                    .and_then(|image| get_image_data(conn, image).ok());

                let store_name = proposal.store_id
                    .and_then(|id| find_store(conn, id).ok().map(|store| store.name));

                StoreProposalDetail {
                    id:proposal.id,
                    proposal_type:proposal.proposal_type,
                    status:proposal.status,
                    description:proposal.description,
                    store_id:proposal.store_id,
                    store_name,
                    image_data
                }
            })
            .collect();

        Ok(proposal_details)
    }
}
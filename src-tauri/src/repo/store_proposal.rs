use crate::DbConnect;
use crate::handler::image_handler::get_image_data;
use crate::model::store_proposal_model::{StoreProposal, StoreProposalDetail};
use diesel::prelude::*;
use crate::handler::store_handler::{close_store, create_new_store};
use crate::schema::store_proposals::dsl::store_proposals;
use crate::schema::store_proposals::{id, status};

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

                StoreProposalDetail {
                    name:proposal.name,
                    id:proposal.id,
                    proposal_type:proposal.proposal_type,
                    status:proposal.status,
                    description:proposal.description,
                    store_id:proposal.store_id,
                    image_data
                }
            })
            .collect();

        Ok(proposal_details)
    }

    pub fn get_store_proposal(conn: &mut DbConnect, proposal_id:i32) -> Result<StoreProposal, String> {
        store_proposals.filter(id.eq(&proposal_id))
            .first(conn)
            .map_err(|e| e.to_string())
    }

    pub fn accept_store_proposal(conn: &mut DbConnect, proposal_id:i32) -> Result<(), String> {
        diesel::update(store_proposals)
            .filter(id.eq(&proposal_id))
            .set(status.eq("Accepted".to_string()))
            .execute(conn)
            .map_err(|e| e.to_string())?;

        let proposal = Self::get_store_proposal(conn, proposal_id)?;

        if proposal.proposal_type == "New" {
            create_new_store(conn, proposal)
        } else {
            close_store(conn, proposal.store_id.unwrap())
        }
    }

    pub fn reject_store_proposal(conn: &mut DbConnect, proposal_id:i32) -> Result<(), String> {
        diesel::update(store_proposals)
            .filter(id.eq(&proposal_id))
            .set(status.eq("Rejected".to_string()))
            .execute(conn)
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
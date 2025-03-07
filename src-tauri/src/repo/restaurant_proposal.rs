use crate::DbConnect;
use crate::handler::image_handler::{create_image, get_image_data};
use diesel::prelude::*;
use diesel::serialize::ToSql;
use crate::model::restaurant_proposal_model::{NewRestaurantProposal, NewRestaurantProposalDetail, RestaurantProposal, RestaurantProposalDetail};
use crate::schema::restaurant_proposals::dsl::restaurant_proposals;

impl RestaurantProposal {
    pub fn propose_new_restaurant(conn: &mut DbConnect, new_proposal:NewRestaurantProposalDetail, image_data:Vec<u8>) -> Result<(), String> {
        let image_id = create_image(conn, image_data, new_proposal.mime_type, new_proposal.image_name)?;

        let proposal = NewRestaurantProposal {
            name:new_proposal.name,
            image_id,
            open_time:new_proposal.open_time,
            close_time:new_proposal.close_time,
            cuisine:new_proposal.cuisine,
            status:"Pending".to_string()
        };

        diesel::insert_into(restaurant_proposals)
            .values(proposal)
            .execute(conn)
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn get_restaurant_proposals(conn: &mut DbConnect) -> Result<Vec<RestaurantProposalDetail>, String> {
        let proposals = restaurant_proposals
            .select(RestaurantProposal::as_select())
            .load(conn)
            .map_err(|e| e.to_string())?;

        let proposal_details: Vec<RestaurantProposalDetail> = proposals
            .into_iter()
            .map(|proposal| {

                let image_data = get_image_data(conn, proposal.image_id).unwrap();

                RestaurantProposalDetail {
                    name:proposal.name,
                    id:proposal.id,
                    status:proposal.status,
                    cuisine:proposal.cuisine,
                    image_data,
                    open_time:proposal.open_time.to_string(),
                    close_time:proposal.close_time.to_string()
                }
            })
            .collect();

        Ok(proposal_details)
    }
}
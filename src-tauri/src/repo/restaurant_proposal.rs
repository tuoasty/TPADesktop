use diesel::RunQueryDsl;
use crate::DbConnect;
use crate::handler::image_handler::create_image;
use crate::model::restaurant_proposal_model::{NewRestaurantProposal, NewRestaurantProposalDetail, RestaurantProposal};
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
}
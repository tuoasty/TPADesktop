use crate::DbConnect;
use crate::model::ride_proposal_model::{NewRideProposal, NewRideProposalDetail, RideProposal, RideProposalDetail};
use diesel::prelude::*;
use crate::handler::image_handler::{create_image, get_image_data};
use crate::handler::ride_handler::{close_ride, create_new_ride};
use crate::schema::ride_proposals::dsl::ride_proposals;
use crate::schema::ride_proposals::{id, status};

impl RideProposal {

    pub fn propose_new_ride(conn: &mut DbConnect, new_proposal:NewRideProposalDetail, image_data:Vec<u8>) -> Result<(), String> {
        let image_id = create_image(conn, image_data, new_proposal.mime_type, new_proposal.image_name)?;

        let proposal = NewRideProposal {
            name:new_proposal.name,
            proposal_type:"New".to_string(),
            status:"Pending".to_string(),
            description:new_proposal.description,
            price:new_proposal.price,
            ride_id:None,
            image_id:Some(image_id)
        };

        diesel::insert_into(ride_proposals)
            .values(proposal)
            .execute(conn)
            .map_err(|e| e.to_string())?;

        Ok(())
    }
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

                RideProposalDetail {
                    id:proposal.id,
                    name:proposal.name,
                    proposal_type:proposal.proposal_type,
                    status:proposal.status,
                    description:proposal.description,
                    price:proposal.price,
                    ride_id:proposal.ride_id,
                    image_data
                }
            })
            .collect();

        Ok(proposal_details)
    }

    pub fn get_ride_proposal(conn: &mut DbConnect, proposal_id:i32) -> Result<RideProposal, String> {
        ride_proposals
            .filter(id.eq(&proposal_id))
            .first(conn)
            .map_err(|e| e.to_string())
    }
    pub fn accept_ride_proposal(conn: &mut DbConnect, proposal_id:i32) -> Result<(), String> {
        diesel::update(ride_proposals)
            .filter(id.eq(&proposal_id))
            .set(status.eq("Accepted".to_string()))
            .execute(conn)
            .map_err(|e| e.to_string())?;

        let proposal = Self::get_ride_proposal(conn, proposal_id)?;

        if proposal.proposal_type == "New" {
            create_new_ride(conn, proposal)
        } else {
            close_ride(conn, proposal.ride_id.unwrap())
        }
    }

    pub fn reject_ride_proposal(conn: &mut DbConnect, proposal_id:i32) -> Result<(), String> {
        diesel::update(ride_proposals)
            .filter(id.eq(&proposal_id))
            .set(status.eq("Rejected".to_string()))
            .execute(conn)
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
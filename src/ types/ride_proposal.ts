export interface RideProposal {
    id:number,
    proposal_type:string,
    status:string,
    description:string,
    ride_id:number | null,
    ride_name:string | null,
    image_data:string | null,
}
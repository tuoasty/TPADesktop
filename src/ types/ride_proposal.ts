export interface RideProposal {
    id:number,
    name:string,
    proposal_type:string,
    status:string,
    description:string,
    price:number,
    ride_id:number | null,
    image_data:string | null,
}
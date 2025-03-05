export interface StoreProposal {
    id:number,
    name:string,
    proposal_type:string,
    status:string,
    description:string,
    store_id:number | null,
    image_data:string | null,
}
export interface StoreProposal {
    id:number,
    proposal_type:string,
    status:string,
    description:string,
    store_id:number | null,
    store_name:string | null,
    image_data:string | null,
}
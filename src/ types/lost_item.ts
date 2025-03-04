export interface LostItem {
    id:number,
    name:string,
    item_type:string,
    color:string,
    last_location:string,
    owner_id:number,
    owner_name:string,
    status:string,
    finder_id: null | number,
    finder_name: null | string,
    found_location:null | string,
    image_data: null | string
}

export interface UpdateLostItemPayload {
    id:number,
    name:string,
    item_type:string,
    color:string,
    last_location:string,
    owner_id:number,
    owner_name:string,
    status:string,
    finder_id: null | number,
    finder_name: null | string,
    found_location:null | string,
    image_data: null | string,
    mime_type: null | string,
    image_name: null | string
}
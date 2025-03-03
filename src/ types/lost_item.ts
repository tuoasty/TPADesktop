export interface LostItem {
    id:number,
    name:string,
    item_type:string,
    color:string,
    last_location:string,
    owner_id:number,
    status:string,
    finder_id: null | number,
    found_location:null | number,
    image_data: null | string
}
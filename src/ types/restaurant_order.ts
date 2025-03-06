export interface RestaurantOrder {
    id:number,
    customer_id:number,
    restaurant_id:number,
    menu_id:number,
    menu_name:string,
    status:string,
    value:number,
    count:number,
    time_ordered:string
}
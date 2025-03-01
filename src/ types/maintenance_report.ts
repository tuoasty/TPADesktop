export interface MaintenanceReport {
    id:number;
    ride_id:number;
    ride_name:string;
    staff_id:number | null;
    staff_name:string | null;
    description:string;
    status:string;
}
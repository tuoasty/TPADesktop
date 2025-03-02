import {Menu} from "@/ types/menu.ts";
import {Staff} from "@/ types/staff.ts";

export interface Restaurant {
    id: number;
    name: string;
    open_time: string;
    close_time: string;
    cuisine: string;
    status: string;
    image_data: string;
    menus: Menu[];
    staffs: Staff[];
}


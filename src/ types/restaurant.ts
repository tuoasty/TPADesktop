import {Menu} from "@/ types/menu.ts";

export interface Restaurant {
    id: number;
    name: string;
    open_time: string;
    close_time: string;
    cuisine: string;
    image_data: string;
    menus: Menu[];
}


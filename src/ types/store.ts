import {Souvenir} from "@/ types/souvenir.ts";

export interface Store {
    id: number;
    name: string;
    open_time: string;
    close_time: string;
    image_data: string;
    souvenirs:Souvenir[];
}
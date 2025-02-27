import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {Restaurant} from "@/ types/restaurant.ts";

export default function ViewAllRestaurant(){
    const [restaurants, setRestaurants] = useState<Restaurant[]>([]);

    useEffect(() => {
        invoke<Restaurant[]>("find_all_restaurant")
            .then(setRestaurants)
    }, []);

    return (
        <div className="h-screen w-full bg-purple-200 flex flex-col p-16 overflow-auto gap-5">
            {restaurants.length > 0 && (
                restaurants.map((restaurant:Restaurant) => (
                    <div key={restaurant.id} className="w-full bg-white h-min-72 rounded-2xl shrink-0 flex">
                        <div className="w-2xl h-full p-8 overflow-hidden">
                            <img className="object-cover w-full h-full rounded-lg" src={restaurant.image_data} alt={restaurant.name}/>
                        </div>
                        <div className="w-full h-full flex flex-col p-8 gap-2">
                            <h1 className="font-bold text-4xl">{restaurant.name}</h1>
                            <h2 className="text-2xl">{restaurant.cuisine}</h2>
                            <h4>{restaurant.open_time} - {restaurant.close_time}</h4>
                        </div>
                    </div>
                ))
            )}
        </div>
    )
}
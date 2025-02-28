import {useEffect, useState} from "react";
import {Ride} from "@/ types/ride.ts";
import {invoke} from "@tauri-apps/api/core";

export default function ViewAllRide(){
    const [rides, setRides] = useState<Ride[]>([])

    const fetchRides = async () => {
        invoke<Ride[]>("find_all_ride").then(setRides);
    }

    useEffect(() => {
        fetchRides();
    }, []);

    return (
        <div className="h-screen w-full bg-purple-200 flex flex-col p-16 overflow-auto gap-5">
            {rides.length > 0 && (
                rides.map((ride:Ride) => (
                    <div key={ride.id} className="w-full bg-white h-min-72 rounded-2xl shrink-0 flex">
                        <div className="w-2xl h-full p-8 overflow-hidden">
                            <img className="object-cover w-full h-full rounded-lg" src={ride.image_data} alt={ride.name}/>
                        </div>
                        <div className="w-full h-full flex flex-col p-8 gap-2">
                            <h1 className="font-bold text-4xl">{ride.name}</h1>
                            <h2 className="text-2xl">Price : {ride.price}</h2>
                            <h4>{ride.open_time} - {ride.close_time}</h4>
                        </div>
                    </div>
                ))
            )}
        </div>
    )
}
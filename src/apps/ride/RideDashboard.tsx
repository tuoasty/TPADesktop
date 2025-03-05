import {useEffect, useState} from "react";
import {Store} from "@/ types/store.ts";
import {invoke} from "@tauri-apps/api/core";
import {toast} from "sonner";
import {useCustomerAuth} from "@/context/CustomerAuthProvider.tsx";
import {useNavigate} from "react-router-dom";
import {Ride} from "@/ types/ride.ts";

interface Props {
    rideId:number;
}
export default function RideDashboard(p:Props){
    const [ride, setRide] = useState<Ride | null>(null);
    const navigate = useNavigate();

    const {name, balance, customerIsLoggedIn} = useCustomerAuth();
    // const purchaseSouvenir = async (souvenirId:number) => {
    //     if (!await customerIsLoggedIn()) {
    //         toast.error("You must be logged in to purchase")
    //         navigate("login")
    //         return;
    //     }
    //
    //     try {
    //         await invoke("purchase_souvenir", {souvenirId:souvenirId, souvenirCount:count});
    //         toast.success("Successfully purchased item");
    //     } catch (e) {
    //         toast.error(`${e}`)
    //     }
    // }

    const fetchRide = async () => {
        try {
            invoke<Store>("find_ride_by_id", {selectedId:p.rideId}).then(setRide);
        } catch (e) {
            toast.error(`${e}`)
        }
    }

    useEffect(() => {
        fetchRide();
    }, []);

    return (
        <main className="bg-red-200 h-full w-full flex flex-col place-items-center justify-center">
            {ride && ride.status == "Open" ? (
                <div className="h-full w-full flex">
                    <div className="w-[25%] h-auto">
                        <img className="object-contain w-full h-full rounded-lg" src={ride.image_data}
                             alt={ride.name}/>
                    </div>
                    <div className="w-full p-12 gap-5">
                        {name && (
                            <div>
                                <h1>Welcome {name}</h1>
                                <h1>Balance : {balance}</h1>
                            </div>
                        )}
                        <h1 className="text-2xl font-bold">{ride.name}</h1>
                    </div>
                </div>
            ) : (
                <h1>Ride is {ride?.status}</h1>
            )}
        </main>
    )
}
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { toast } from "sonner";
import { useCustomerAuth } from "@/context/CustomerAuthProvider.tsx";
import { useNavigate } from "react-router-dom";
import { Button } from "@/components/ui/button.tsx";
import {Ride} from "@/ types/ride.ts";

interface Props {
    rideId: number;
}

export default function RideDashboard({ rideId }: Props) {
    const [ride, setRide] = useState<Ride | null>(null);
    const navigate = useNavigate();
    const { customerId, name, balance, customerIsLoggedIn } = useCustomerAuth();

    const addCustomerToRideQueue = async () => {
        try {
            if (!(await customerIsLoggedIn())) {
                toast.error("You must be logged in to purchase");
                navigate("login");
                return;
            }
            await invoke("add_customer_to_ride_queue", { selectedRideId: rideId, selectedCustomerId: customerId });
            toast.success("Successfully queued for the ride!");
        } catch (e) {
            toast.error(`${e}`);
        }
    };

    const fetchRide = async () => {
        try {
            invoke<Ride>("find_ride_by_id", { selectedId: rideId }).then(setRide);
        } catch (e) {
            toast.error(`${e}`);
        }
    };

    useEffect(() => {
        fetchRide();
    }, []);

    return (
        <main className="bg-red-100 min-h-screen w-full flex items-center justify-center p-6">
            {ride && ride.status === "Open" ? (
                <div className="bg-white shadow-lg rounded-xl overflow-hidden w-full max-w-4xl flex flex-col md:flex-row p-6 gap-6">
                    <div className="w-full md:w-1/3">
                        <img className="object-cover w-full h-64 rounded-lg" src={ride.image_data} alt={ride.name} />
                    </div>

                    <div className="w-full md:w-2/3 flex flex-col justify-between gap-4">
                        {name && (
                            <div className="text-gray-800">
                                <h1 className="text-lg font-semibold">Welcome, {name}!</h1>
                                <p className="text-gray-600">Balance: <span className="font-bold">{balance}</span></p>
                            </div>
                        )}

                        <div className="space-y-2">
                            <h1 className="text-3xl font-bold text-gray-900">{ride.name}</h1>
                            <p className="text-lg text-gray-700">Price: <span className="font-semibold">{ride.price}</span></p>
                        </div>

                        <Button className="w-full md:w-auto bg-red-500 hover:bg-red-600 text-white font-bold px-6 py-3 rounded-lg"
                                onClick={addCustomerToRideQueue}>
                            Queue for Ride
                        </Button>
                    </div>
                </div>
            ) : (
                <h1 className="text-2xl font-semibold text-gray-700">Ride is {ride?.status || "Unavailable"}</h1>
            )}
        </main>
    );
}
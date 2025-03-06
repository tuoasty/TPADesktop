import {useStaffAuth} from "@/context/StaffAuthProvider.tsx";
import {useEffect, useState} from "react";
import {Ride} from "@/ types/ride.ts";
import {invoke} from "@tauri-apps/api/core";
import {toast} from "sonner";
import {Staff} from "@/ types/staff.ts";
export default function ViewRide() {
    const {staffId} = useStaffAuth();
    const [ride, setRide] = useState<Ride | null>(null)

    const fetchRideStaffRide = async () => {
        try {
            invoke<Ride>("find_staff_ride", {selectedId: staffId}).then(setRide)
        } catch (e) {
            toast.error(`${e}`)
        }
    }

    useEffect(() => {
        fetchRideStaffRide();
    }, []);

    return (
        <div className="h-screen w-full bg-purple-200 flex flex-col p-16 overflow-auto gap-5">
            {ride ? (
                <div className="w-full bg-white h-auto rounded-2xl shrink-0 flex">
                    <div className="w-96 h-auto p-8 overflow-hidden">
                        <img className="object-contain w-full h-full rounded-lg" src={ride.image_data}
                             alt={ride.name}/>
                    </div>
                    <div className="w-full h-auto flex flex-col p-8 gap-5">
                        <div className="w-full h-full justify-between flex flex-row">
                            <div className="w-auto flex flex-col gap-2">
                                <h1 className="font-bold text-4xl">{ride.name}</h1>
                                <h2 className="text-2xl">Price : {ride.price}</h2>
                                <h4>{ride.open_time} - {ride.close_time}</h4>
                                <h3>Status : {ride.status}</h3>
                            </div>
                        </div>
                        <div>
                            <h1 className="font-bold text-2xl">Staffs</h1>
                            {ride.staffs.length == 0 ? (
                                <h2>None</h2>
                            ) : ride.staffs.map((staff: Staff) => (
                                <div key={staff.id}>
                                    <h2>{staff.name}</h2>
                                </div>
                            ))}
                        </div>
                    </div>
                </div>
            ) : (
                <h1>You are not assigned to any ride</h1>
            )}
        </div>
    )
}
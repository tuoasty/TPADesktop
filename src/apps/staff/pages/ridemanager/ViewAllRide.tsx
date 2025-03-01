import {useEffect, useState} from "react";
import {Ride} from "@/ types/ride.ts";
import {invoke} from "@tauri-apps/api/core";
import {Button} from "@/components/ui/button.tsx";
import {
    Dialog,
    DialogTrigger,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogContent
} from "@/components/ui/dialog.tsx";
import {Label} from "@/components/ui/label.tsx";
import {Input} from "@/components/ui/input.tsx";
import {toast} from "sonner";

export default function ViewAllRide(){
    const [rides, setRides] = useState<Ride[]>([])

    const fetchRides = async () => {
        invoke<Ride[]>("find_all_ride").then(setRides);
    }

    useEffect(() => {
        fetchRides();
    }, []);

    const updateStatus = (id:number, status:string) => {
        try {
            invoke("change_ride_status", {rideId:id, rideStatus:status}).then(() => {
                toast.success("Successfully updated ride status");
                fetchRides();
            })

        } catch (e) {
            toast.error(`${e}`)
        }
    }

    return (
        <div className="h-screen w-full bg-purple-200 flex flex-col p-16 overflow-auto gap-5">
            {rides.length > 0 && (
                rides.map((ride:Ride) => (
                    <div key={ride.id} className="w-full bg-white h-min-72 rounded-2xl shrink-0 flex">
                        <div className="w-96 h-auto p-8 overflow-hidden">
                            <img className="object-contain w-full h-full rounded-lg" src={ride.image_data}
                                 alt={ride.name}/>
                        </div>
                        <div className="w-full h-full justify-between flex flex-row">
                            <div className="w-auto flex flex-col p-8 gap-2">
                                <h1 className="font-bold text-4xl">{ride.name}</h1>
                                <h2 className="text-2xl">Price : {ride.price}</h2>
                                <h4>{ride.open_time} - {ride.close_time}</h4>
                                <h3>Status : {ride.status}</h3>
                            </div>
                            <div className="w-48 flex justify-center place-items-center mr-6 flex-col gap-4">
                                <Dialog>
                                    <DialogTrigger asChild>
                                        <Button disabled={ride.status == "Maintenance in Progress" || ride.status == "Pending Maintenance"}
                                                className="bg-purple-700 w-48 h-12">Request Maintenance</Button>
                                    </DialogTrigger>
                                    <DialogContent>
                                        <DialogHeader>
                                            <DialogTitle>Maintenance Request</DialogTitle>
                                            <DialogDescription>Enter maintenance description</DialogDescription>
                                        </DialogHeader>
                                        <div className="grid gap-4 py-4">
                                            <div className="grid grid-cols-4 items-center gap-4">
                                                <Label htmlFor="reason" className="text-right">
                                                    Reasoning
                                                </Label>
                                                <Input id="reason" type="text" className="col-span-3" />
                                            </div>
                                        </div>
                                        <DialogFooter>
                                            <Button type="submit" className="bg-purple-700">Save changes</Button>
                                        </DialogFooter>
                                    </DialogContent>
                                </Dialog>
                                <Button className={`w-48 h-12  ${ride.status == "Closed" ? "bg-green-500" : ride.status == "Open" ? "bg-red-500" : "bg-purple-700"}`}
                                        disabled={ride.status !== "Open" && ride.status !== "Closed"}
                                onClick={() => updateStatus(ride.id, ride.status)}>
                                    {ride.status === "Open" ? "Close" : "Open"}
                                </Button>
                            </div>
                        </div>
                    </div>
                ))
            )}
        </div>
    )
}
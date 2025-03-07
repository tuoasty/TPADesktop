import {useEffect, useState} from "react";
import {Ride} from "@/ types/ride.ts";
import {invoke} from "@tauri-apps/api/core";
import {Button} from "@/components/ui/button.tsx";
import {toast} from "sonner";
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from "@/components/ui/select.tsx";
import {Input} from "@/components/ui/input.tsx";

export default function CreateMaintenanceTask() {
    const [rides, setRides] = useState<Ride[]>([])
    const [selectedRide, setSelectedRide] = useState<number | null>(null);
    const [newMaintenanceReason, setNewMaintenanceReason] = useState("");

    const fetchRides = async () => {
        invoke<Ride[]>("find_all_ride").then(setRides);
    }
    useEffect(() => {
        fetchRides();
    }, []);

    const reportRideMaintenance = async (rideId:number, reason: string) => {
        try {
            await invoke("report_ride_maintenance", {rideId: rideId, description: reason})
            toast.success("Successfully added maintenance")
            fetchRides()
        } catch (e) {
            toast.error(`${e}`)
        }
    }

    return (
        <div className="h-screen w-full bg-purple-200 flex flex-col p-16 overflow-auto gap-5">
            <div className="w-full bg-white p-8 rounded-2xl shadow-md">
                <h2 className="text-2xl font-bold mb-4">Create Maintenance Task</h2>
                <div className="flex flex-col gap-4">
                    <Select onValueChange={(val) => setSelectedRide(Number(val))}>
                        <SelectTrigger>
                            <SelectValue placeholder="Select a Ride" />
                        </SelectTrigger>
                        <SelectContent>
                            {rides.length > 0 && (
                                rides.map((ride: Ride) => (
                                    <SelectItem key={ride.id} value={ride.id.toString()}>{ride.name}</SelectItem>
                                ))
                            )}
                        </SelectContent>
                    </Select>
                    <Input
                        type="text"
                        placeholder="Maintenance details"
                        onChange={(e) => setNewMaintenanceReason(e.target.value)}
                    />
                    <Button
                        className="bg-purple-700"
                        onClick={() => {
                            if (selectedRide && newMaintenanceReason) {
                                reportRideMaintenance(selectedRide, newMaintenanceReason);
                            } else {
                                toast.error("Please select a ride and enter maintenance details.");
                            }
                        }}
                    >
                        Create Maintenance Task
                    </Button>
                </div>
            </div>
        </div>
    )
}

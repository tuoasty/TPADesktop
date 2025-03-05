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

import {toast} from "sonner";
import {Staff} from "@/ types/staff.ts";
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from "@/components/ui/select.tsx";
import {
    AlertDialog,
    AlertDialogAction,
    AlertDialogCancel,
    AlertDialogContent,
    AlertDialogDescription,
    AlertDialogFooter,
    AlertDialogHeader,
    AlertDialogTitle,
} from "@/components/ui/alert-dialog.tsx";
import {Label} from "@/components/ui/label.tsx";
import {Input} from "@/components/ui/input.tsx";

export default function ViewAllRide() {
    const [rides, setRides] = useState<Ride[]>([])
    const [staffs, setStaffs] = useState<Staff[]>([])
    const [selectedId, setSelectedId] = useState<number | null>(null);
    const [rideId, setRideId] = useState<number | null>(null);
    const [reassignDialog, setReassignDialog] = useState(false);

    const fetchRides = async () => {
        invoke<Ride[]>("find_all_ride").then(setRides);
    }

    const fetchRideStaffs = async () => {
        invoke<Staff[]>("find_all_staff", {staffRole: "Ride Staff"})
            .then(setStaffs)
    }

    useEffect(() => {
        fetchRides();
        fetchRideStaffs();
    }, []);

    const assignStaffToRide = async (rideId: number) => {
        try {
            await invoke("assign_staff_to_ride", {staffId: selectedId, rideId: rideId})
            toast.success("Successfully assigned staff");
            fetchRides();
        } catch (e) {
            const error = `${e}`;

            if (error.includes("STAFF ASSIGNED")) {
                setRideId(rideId);
                setReassignDialog(true);
            } else {
                toast.error(error)
            }
        }
    }

    const reassignRideAndCheckStatus = async () => {
        try {
            await invoke("reassign_ride_and_check_status", {newStaffId: selectedId, newRideId: rideId})
            toast.success("Succesfully reassigned staff")
            fetchRides();
        } catch (e) {
            toast.error(`${e}`)
        }
    }

    const changeRideStatus = async (id: number, status: string) => {
        try {
            await invoke("change_ride_status", {rideId: id, rideStatus: status})
            toast.success("Successfully updated ride status");
            fetchRides();
        } catch (e) {
            toast.error(`${e}`)
        }
    }

    return (
        <div className="h-screen w-full bg-purple-200 flex flex-col p-16 overflow-auto gap-5">
            {rides.length > 0 && (
                rides.map((ride: Ride) => (
                    ride.status != "Shut Down" && (
                        <div key={ride.id} className="w-full bg-white h-auto rounded-2xl shrink-0 flex">
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
                                    <div className="w-48 flex justify-center place-items-center mr-6 flex-col gap-4">
                                        <Dialog>
                                            <DialogTrigger asChild>
                                                <Button
                                                    disabled={ride.status == "Maintenance in Progress" || ride.status == "Pending Maintenance"}
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
                                                        <Input id="reason" type="text" className="col-span-3"/>
                                                    </div>
                                                </div>
                                                <DialogFooter>
                                                    <Button type="submit" className="bg-purple-700">Save changes</Button>
                                                </DialogFooter>
                                            </DialogContent>
                                        </Dialog>
                                        <Dialog>
                                            <DialogTrigger asChild>
                                                <Button className="bg-purple-700 w-48 h-12">Assign Staff</Button>
                                            </DialogTrigger>
                                            <DialogContent>
                                                <DialogHeader>
                                                    <DialogTitle>Ride Staff</DialogTitle>
                                                    <DialogDescription>Choose staff to assign.
                                                        Rides need at least two staff</DialogDescription>
                                                </DialogHeader>
                                                <Select onValueChange={(val) => setSelectedId(Number(val))}>
                                                    <SelectTrigger>
                                                        <SelectValue placeholder="Ride Staff"/>
                                                    </SelectTrigger>
                                                    <SelectContent>
                                                        {staffs.length > 0 && (
                                                            staffs.map((staff: Staff) => (
                                                                <SelectItem key={staff.id}
                                                                            value={staff.id.toString()}>{staff.name}</SelectItem>
                                                            ))
                                                        )}
                                                    </SelectContent>
                                                </Select>
                                                <DialogFooter>
                                                    <DialogTrigger asChild>
                                                        <Button
                                                            onClick={() => assignStaffToRide(ride.id)}
                                                            type="submit"
                                                            className="bg-purple-700">
                                                            Confirm
                                                        </Button>
                                                    </DialogTrigger>
                                                </DialogFooter>
                                            </DialogContent>
                                        </Dialog>
                                        <AlertDialog open={reassignDialog} onOpenChange={setReassignDialog}>
                                            <AlertDialogContent>
                                                <AlertDialogHeader>
                                                    <AlertDialogTitle>Staff Already Assigned</AlertDialogTitle>
                                                    <AlertDialogDescription>
                                                        This staff member is already assigned.
                                                        Do you want to reassign them?
                                                    </AlertDialogDescription>
                                                </AlertDialogHeader>
                                                <AlertDialogFooter>
                                                    <AlertDialogCancel>Cancel</AlertDialogCancel>
                                                    <AlertDialogAction onClick={() => reassignRideAndCheckStatus()}>
                                                        Reassign
                                                    </AlertDialogAction>
                                                </AlertDialogFooter>
                                            </AlertDialogContent>
                                        </AlertDialog>
                                        <Button
                                            className={`w-48 h-12  ${ride.status == "Closed" ? "bg-green-500" : ride.status == "Open" ? "bg-red-500" : "bg-purple-700"}`}
                                            disabled={ride.status !== "Open" && ride.status !== "Closed"}
                                            onClick={() => changeRideStatus(ride.id, ride.status)}>
                                            {ride.status === "Open" ? "Close" : "Open"}
                                        </Button>
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
                    )
                ))
            )}
        </div>
    )
}
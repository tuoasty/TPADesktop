import {useStaffAuth} from "@/context/StaffAuthProvider.tsx";
import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {toast} from "sonner";
import {Dialog, DialogContent, DialogTrigger} from "@/components/ui/dialog";
import {Button} from "@/components/ui/button";
import {Input} from "@/components/ui/input";
import {Ride} from "@/ types/ride.ts";
import {RideQueue} from "@/ types/ride_queue.ts";
import {Staff} from "@/ types/staff.ts";

export default function ViewRide() {
    const {staffId} = useStaffAuth();
    const [ride, setRide] = useState<Ride | null>(null);
    const [queues, setQueues] = useState<RideQueue[]>([]);
    const [customerId, setCustomerId] = useState(0);

    const fetchRideStaffRide = async () => {
        try {
            let rides = await invoke<Ride>("find_staff_ride", {selectedId: staffId});
            setRide(rides);
        } catch (e) {
            toast.error(`${e}`);
        }
    };

    const fetchRideQueue = async () => {
        if (!ride) return;
        try {
            let queueData = await invoke<RideQueue[]>("find_ride_queue", {selectedId: ride.id});
            setQueues(queueData);
        } catch (e) {
            toast.error(`${e}`);
        }
    };

    const dequeueCustomer = async (id: number) => {
        try {
            await invoke("dequeue_customer_from_ride", {selectedRideId:ride?.id, selectedCustomerId:id});
            toast.success("Customer dequeued successfully");
            fetchRideQueue();
        } catch (e) {
            toast.error(`${e}`);
        }
    };

    const queueCustomer = async () => {
        try {
            await invoke("add_customer_to_ride_queue", {selectedRideId: ride?.id, selectedCustomerId:customerId});
            toast.success("Customer added to queue");
            setCustomerId(0);
            fetchRideQueue();
        } catch (e) {
            toast.error(`${e}`);
        }
    };

    useEffect(() => {
        fetchRideStaffRide();
    }, []);

    useEffect(() => {
        fetchRideQueue();
    }, [ride]);

    return (
        <div className="h-screen w-full bg-purple-200 flex flex-col p-16 overflow-auto gap-5">
            {ride ? (
                <div className="w-full bg-white h-auto rounded-2xl shrink-0 flex">
                    <div className="w-96 h-auto p-8 overflow-hidden">
                        <img className="object-contain w-full h-full rounded-lg" src={ride.image_data} alt={ride.name}/>
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
                            {ride.staffs.length === 0 ? (
                                <h2>None</h2>
                            ) : ride.staffs.map((staff: Staff) => (
                                <div key={staff.id}>
                                    <h2>{staff.name}</h2>
                                </div>
                            ))}
                        </div>
                        <div>
                            <h1 className="font-bold text-2xl">Queue</h1>
                            {queues.length <= 0 ? (
                                <h1>No Queue</h1>
                            ) : (
                                queues.map((queue: RideQueue) => (
                                    <div key={queue.id} className="flex justify-between p-2 bg-gray-100 rounded-lg mb-2">
                                        <h1>{queue.customer_name}</h1>
                                        <h1>{queue.status}</h1>
                                        <Button disabled={queue.status == "Completed"} onClick={() => dequeueCustomer(queue.customer_id)}>Dequeue</Button>
                                    </div>
                                ))
                            )}
                        </div>
                        <div>
                            <Dialog>
                                <DialogTrigger>
                                    <Button>Queue Customer</Button>
                                </DialogTrigger>
                                <DialogContent>
                                    <h2 className="text-lg font-bold">Add Customer to Queue</h2>
                                    <Input placeholder="Enter Customer ID" value={customerId} onChange={(e) => setCustomerId(parseInt(e.target.value, 0))} />
                                    <Button onClick={queueCustomer}>Add to Queue</Button>
                                </DialogContent>
                            </Dialog>
                        </div>
                    </div>
                </div>
            ) : (
                <h1>You are not assigned to any ride</h1>
            )}
        </div>
    );
}
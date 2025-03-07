import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {Button} from "@/components/ui/button.tsx";
import {toast} from "sonner";
import {RestaurantProposal} from "@/ types/restaurant_proposal.ts";
import {useStaffAuth} from "@/context/StaffAuthProvider.tsx";

export default function ViewRestaurantProposal() {
    const {role} = useStaffAuth();
    const [restaurantProposals, setRestaurantProposals] = useState<RestaurantProposal[]>([]);

    const fetchRestaurantProposals = async () => {
        try {
            const proposals = await invoke<RestaurantProposal[]>("find_restaurant_proposals");
            setRestaurantProposals(proposals);
        } catch (error) {
            console.error("Error fetching restaurant proposals:", error);
        }
    };

    const acceptRestaurantProposal = async (proposalId: number, currentStatus:string) => {
        try {
            await invoke("accept_restaurant_proposal", {proposalId:proposalId, role:role, currentStatus:currentStatus});
            toast.success("Success")
            fetchRestaurantProposals();
        } catch (e) {
            toast.error(`${e}`)
        }
    }

    const rejectRestaurantProposal = async (proposalId: number) => {
        try {
            // await invoke("reject_ride_proposal", {proposalId:proposalId});
            toast.success("Success")
            fetchRestaurantProposals();
        } catch (e) {
            toast.error(`${e}`)
        }
    }

    useEffect(() => {
        fetchRestaurantProposals();
    }, []);

    return (
        <div className="h-screen w-full bg-purple-200 flex flex-col p-16 overflow-auto gap-5">
            {restaurantProposals.length > 0 ? (
                restaurantProposals.map((proposal: RestaurantProposal) => (
                    <div key={proposal.id} className="w-full bg-white min-h-72 rounded-2xl flex">
                        <div className="w-96 h-auto p-8 overflow-hidden">
                            <img className="object-contain w-full h-full rounded-lg" src={proposal.image_data}
                                 alt={proposal.name}/>
                        </div>
                        <div className="w-full flex flex-row p-8 gap-2 justify-between">
                            <div className="flex flex-col">
                                <h1 className="font-bold text-4xl">New Store</h1>
                                <h1>{proposal.name}</h1>
                                <h2>Cuisine: {proposal.cuisine}</h2>
                                <h2>Open Time: {proposal.open_time}</h2>
                                <h2>Close Time: {proposal.close_time}</h2>
                                <h2>Status: {proposal.status}</h2>
                            </div>
                            <div className="flex flex-col gap-5 justify-center pr-8">
                                <Button className="w-48" onClick={() => {
                                    acceptRestaurantProposal(proposal.id, proposal.status)
                                }}
                                        disabled={proposal.status == "Accepted" || proposal.status == "Rejected" || proposal.status == `Accepted by ${role}`}>Accept</Button>
                                <Button className="w-48" onClick={() => {
                                    rejectRestaurantProposal(proposal.id)
                                }}
                                        disabled={proposal.status == "Accepted" || proposal.status == "Rejected" || proposal.status == `Accepted by ${role}`}>Reject</Button>
                            </div>
                        </div>
                    </div>
                ))
            ) : (
                <h2 className="text-center text-2xl font-bold">No restaurant proposals found.</h2>
            )}
        </div>
    );
}

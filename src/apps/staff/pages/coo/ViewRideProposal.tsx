import {useEffect, useState} from "react";
import {RideProposal} from "@/ types/ride_proposal.ts";
import {invoke} from "@tauri-apps/api/core";
import {Button} from "@/components/ui/button.tsx";

export default function ViewRideProposal() {

    const [rideProposals, setRideProposals] = useState<RideProposal[]>([]);

    const fetchRideProposals = async () => {
        invoke<RideProposal[]>("find_ride_proposal").then(setRideProposals)
    }

    useEffect(() => {
        fetchRideProposals()
    }, []);

    return (
        <div className="h-screen w-full bg-purple-200 flex flex-col p-16 overflow-auto gap-5">
            {/*    Map for all ride proposal*/}
            {rideProposals.length > 0 && (
                rideProposals.map((proposal: RideProposal) => (
                    <div className="w-full bg-white h-min-72 rounded-2xl shrink-0 flex">
                        <div className="w-96 h-auto p-8 overflow-hidden">
                            {proposal.proposal_type == "Remove" || !proposal.image_data ? (
                                <div className="flex justify-center place-items-center h-full">
                                    <h1>No Image</h1>
                                </div>
                            ) : (
                                <img className="object-contain w-full h-full rounded-lg" src={proposal.image_data}
                                     alt={proposal.description}/>
                            )}
                        </div>
                        <div className="w-full h-full flex flex-row p-8 gap-2 justify-between">
                            <div className="w-auto h-full flex-col flex">
                                <h1 className="font-bold text-4xl">{proposal.proposal_type} Ride</h1>
                                {proposal.proposal_type == "Remove" && (
                                    <h1>{proposal.ride_name}</h1>
                                )}
                                <h2>Description : {proposal.description}</h2>
                                <h2>Status : {proposal.status}</h2>
                            </div>
                            <div className="flex flex-col gap-5 justify-center pr-8">
                                <Button className="w-48">Accept</Button>
                                <Button className="w-48">Reject</Button>
                            </div>
                        </div>
                    </div>
                ))
            )}
        </div>
    )
}
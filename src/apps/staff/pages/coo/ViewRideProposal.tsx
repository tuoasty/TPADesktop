import {useEffect, useState} from "react";
import {RideProposal} from "@/ types/ride_proposal.ts";
import {invoke} from "@tauri-apps/api/core";

export default function ViewRideProposal() {

    const [rideProposals, setRideProposals] = useState<RideProposal[]>([]);

    const fetchRideProposals = async() => {
        invoke<RideProposal[]>("find_ride_proposal").then(setRideProposals)
    }

    useEffect(() => {
        fetchRideProposals()
    }, []);

    return (
        <div className="h-screen w-full bg-purple-200 flex flex-col p-16 overflow-auto gap-5">
        {/*    Map for all ride proposal*/}
            {rideProposals.length > 0 && (
                rideProposals.map((proposal:RideProposal) => (
                    <div className="w-full bg-white h-min-72 rounded-2xl shrink-0 flex justify-between">
                        <div className="w-auto h-full flex flex-col p-8 gap-2">
                            <h1 className="font-bold text-4xl">{proposal.proposal_type} Ride</h1>
                        </div>
                        <div className="flex flex-col gap-5 justify-center pr-8">
                            {/*Buttons*/}
                        </div>
                    </div>
                ))
            )}
        </div>
    )
}
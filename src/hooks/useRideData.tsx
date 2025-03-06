import { useQuery } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/core';
import {Ride} from "@/ types/ride.ts";

export function useRideData(rideId: number) {
    return useQuery({
        queryKey: ['ride', rideId],
        queryFn: () => invoke<Ride>("find_ride_by_id", { selectedId: rideId }),
    });
}
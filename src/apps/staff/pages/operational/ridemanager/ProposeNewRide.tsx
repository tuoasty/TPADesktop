import { Input } from "@/components/ui/input.tsx";
import { Label } from "@/components/ui/label.tsx";
import { Button } from "@/components/ui/button.tsx";
import React, { useState } from "react";
import { toast } from "sonner";
import { createImageFile } from "@/lib/fileFunctions.ts";
import { invoke } from "@tauri-apps/api/core";
import {NewImage} from "@/ types/image.ts";

export default function ProposeNewRide() {
    const [rideData, setRideData] = useState({
        name: "",
        description: "",
        price: 0,
        imageData: "",
        imageName: "",
        mimeType: "",
    });

    async function handleFileChange(e: React.ChangeEvent<HTMLInputElement>) {
        const image: NewImage | undefined = await createImageFile(e)!;
        if (!image) return;

        setRideData((prev) => ({
            ...prev,
            imageData: image.imageData,
            imageName: image.fileName,
            mimeType: image.mimeType,
        }));
    }

    async function createRideProposal() {
        if (!rideData.name || !rideData.description || !rideData.price) {
            toast.error("Please fill in all fields.");
            return;
        }

        const proposalData = {
            name: rideData.name,
            description: rideData.description,
            price: rideData.price,
            image_data: rideData.imageData,
            mime_type: rideData.mimeType,
            image_name: rideData.imageName,
        };

        console.log("Submitting Ride Proposal:", proposalData);

        try {
            await invoke("propose_new_ride", { newProposal: proposalData });
            toast.success("Successfully proposed a new ride!");
        } catch (e) {
            toast.error(`Error: ${e}`);
        }
    }

    return (
        <div className="h-full w-full bg-purple-200 flex justify-center items-center">
            <div className="bg-white h-auto w-[25%] flex flex-col justify-center items-center gap-5 p-12 rounded-2xl">
                <h1 className="font-bold text-2xl">Propose New Ride</h1>

                <Input
                    type="text"
                    placeholder="Ride Name"
                    value={rideData.name}
                    onChange={(e) => setRideData({ ...rideData, name: e.target.value })}
                    required
                />

                <Input
                    type="text"
                    placeholder="Description"
                    value={rideData.description}
                    onChange={(e) => setRideData({ ...rideData, description: e.target.value })}
                    required
                />

                <Input
                    type="number"
                    placeholder="Price"
                    value={rideData.price}
                    onChange={(e) => setRideData({ ...rideData, price: parseInt(e.target.value, 0) })}
                    required
                />

                <div className="grid w-full max-w-sm items-center gap-1.5">
                    <Label htmlFor="picture">Picture</Label>
                    <Input type="file" id="picture" accept="image/*" onChange={handleFileChange} />
                </div>

                <Button className="bg-purple-500 w-full" onClick={createRideProposal}>
                    Propose Ride
                </Button>
            </div>
        </div>
    );
}
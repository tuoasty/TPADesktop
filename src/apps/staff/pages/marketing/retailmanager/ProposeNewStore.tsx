import { Input } from "@/components/ui/input.tsx";
import { Label } from "@/components/ui/label.tsx";
import { Button } from "@/components/ui/button.tsx";
import React, { useState } from "react";
import { toast } from "sonner";
import { createImageFile } from "@/lib/fileFunctions.ts";
import { invoke } from "@tauri-apps/api/core";
import {NewImage} from "@/ types/image.ts";

export default function ProposeNewStore() {
    const [storeData, setStoreData] = useState({
        name: "",
        description: "",
        imageData: "",
        imageName: "",
        mimeType: "",
    });

    async function handleFileChange(e: React.ChangeEvent<HTMLInputElement>) {
        const image: NewImage | undefined = await createImageFile(e)!;
        if (!image) return;

        setStoreData((prev) => ({
            ...prev,
            imageData: image.imageData,
            imageName: image.fileName,
            mimeType: image.mimeType,
        }));
    }

    async function createStoreProposal() {
        if (!storeData.name || !storeData.description) {
            toast.error("Please fill in all fields.");
            return;
        }

        const proposalData = {
            name: storeData.name,
            description: storeData.description,
            image_data: storeData.imageData,
            mime_type: storeData.mimeType,
            image_name: storeData.imageName,
        };

        console.log("Submitting Store Proposal:", proposalData);

        try {
            await invoke("propose_new_store", { newProposal: proposalData });
            toast.success("Successfully proposed a new store!");
        } catch (e) {
            toast.error(`Error: ${e}`);
        }
    }

    return (
        <div className="h-full w-full bg-purple-200 flex justify-center items-center">
            <div className="bg-white h-auto w-[25%] flex flex-col justify-center items-center gap-5 p-12 rounded-2xl">
                <h1 className="font-bold text-2xl">Propose New Store</h1>

                <Input
                    type="text"
                    placeholder="Store Name"
                    value={storeData.name}
                    onChange={(e) => setStoreData({ ...storeData, name: e.target.value })}
                    required
                />

                <Input
                    type="text"
                    placeholder="Description"
                    value={storeData.description}
                    onChange={(e) => setStoreData({ ...storeData, description: e.target.value })}
                    required
                />

                <div className="grid w-full max-w-sm items-center gap-1.5">
                    <Label htmlFor="picture">Picture</Label>
                    <Input type="file" id="picture" accept="image/*" onChange={handleFileChange} />
                </div>

                <Button className="bg-purple-500 w-full" onClick={createStoreProposal}>
                    Propose Store
                </Button>
            </div>
        </div>
    );
}

import { Input } from "@/components/ui/input.tsx";
import { Label } from "@/components/ui/label.tsx";
import { Button } from "@/components/ui/button.tsx";
import React, { useState } from "react";
import { toast } from "sonner";
import { createImageFile } from "@/lib/fileFunctions.ts";
import { invoke } from "@tauri-apps/api/core";
import {NewImage} from "@/ types/image.ts";

export default function ProposeNewRestaurant() {
    const [formData, setFormData] = useState({
        name: "",
        cuisine: "",
        open_time: "",
        close_time: "",
        image_data: "",
        image_name: "",
        mime_type: "",
    });

    async function handleFileChange(e: React.ChangeEvent<HTMLInputElement>) {
        const image: NewImage | undefined = await createImageFile(e)!;
        if (!image) return;

        setFormData({
            ...formData,
            image_data: image.imageData,
            image_name: image.fileName,
            mime_type: image.mimeType,
        });
    }

    function handleInputChange(e: React.ChangeEvent<HTMLInputElement>) {
        setFormData({ ...formData, [e.target.name]: e.target.value });
    }

    async function proposeNewRestaurant() {
        console.log("Submitting Form Data:", formData);

        try {
            await invoke("propose_new_restaurant", { newProposal: formData });
            toast.success("Successfully proposed a new restaurant");
        } catch (e) {
            toast.error(`Error: ${e}`);
        }
    }

    return (
        <div className="h-full w-full bg-purple-200 flex justify-center place-items-center">
            <div className="bg-white h-auto w-[25%] flex flex-col justify-center place-items-center gap-5 p-12 rounded-2xl">
                <h1 className="font-bold text-2xl">Propose New Restaurant</h1>
                <Input
                    type="text"
                    placeholder="Restaurant Name"
                    name="name"
                    value={formData.name}
                    onChange={handleInputChange}
                />
                <Input
                    type="text"
                    placeholder="Cuisine"
                    name="cuisine"
                    value={formData.cuisine}
                    onChange={handleInputChange}
                />
                <div className="grid w-full max-w-sm items-center gap-1.5">
                    <Label htmlFor="openTime">Opening Time</Label>
                    <Input
                        type="time"
                        id="open_time"
                        name="open_time"
                        value={formData.open_time}
                        onChange={handleInputChange}
                    />
                </div>
                <div className="grid w-full max-w-sm items-center gap-1.5">
                    <Label htmlFor="closeTime">Closing Time</Label>
                    <Input
                        type="time"
                        id="close_time"
                        name="close_time"
                        value={formData.close_time}
                        onChange={handleInputChange}
                    />
                </div>
                <div className="grid w-full max-w-sm items-center gap-1.5">
                    <Label htmlFor="picture">Picture</Label>
                    <Input type="file" id="picture" accept="image/*" onChange={handleFileChange} />
                </div>
                <Button className="bg-purple-500 w-full" type="button" onClick={proposeNewRestaurant}>
                    Propose Restaurant
                </Button>
            </div>
        </div>
    );
}

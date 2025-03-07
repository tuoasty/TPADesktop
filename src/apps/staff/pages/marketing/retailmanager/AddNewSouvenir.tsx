import { Input } from "@/components/ui/input.tsx";
import { Label } from "@/components/ui/label.tsx";
import { Button } from "@/components/ui/button.tsx";
import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select.tsx";
import { toast } from "sonner";
import { createImageFile } from "@/lib/fileFunctions.ts";
import {NewImage} from "@/ types/image.ts";
import {Store} from "@/ types/store.ts";

interface SouvenirFormData {
    name: string;
    description: string;
    price: number;
    storeId: number;
    imageData: string;
    imageName: string;
    mimeType: string;
}

export default function AddNewSouvenir() {
    const [stores, setStores] = useState<Store[]>([]);
    const [formData, setFormData] = useState<SouvenirFormData>({
        name: "",
        description: "",
        price: 0,
        storeId: 0,
        imageData: "",
        imageName: "",
        mimeType: "",
    });

    useEffect(() => {
        invoke<Store[]>("find_all_store").then(setStores);
    }, []);

    async function handleFileChange(e: React.ChangeEvent<HTMLInputElement>) {
        const image: NewImage | undefined = await createImageFile(e)!;

        if (!image) {
            return;
        }

        setFormData({
            ...formData,
            imageData: image.imageData,
            imageName: image.fileName,
            mimeType: image.mimeType,
        });
    }

    function handleInputChange(e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) {
        setFormData({ ...formData, [e.target.name]: e.target.value });
    }

    async function createSouvenir(e: React.FormEvent) {
        e.preventDefault();

        const souvenirData = {
            name: formData.name,
            description: formData.description,
            store_id: formData.storeId,
            price: Number(formData.price),
            image_data: formData.imageData,
            mime_type: formData.mimeType,
            image_name: formData.imageName,
        };

        try {
            await invoke("create_souvenir", { souvenir: souvenirData });
            toast.success("Successfully added new souvenir!");
        } catch (error) {
            toast.error(`${error}`);
        }
    }

    return (
        <div className="h-full w-full bg-purple-200 flex justify-center place-items-center">
            <form
                className="bg-white h-auto w-[25%] flex flex-col justify-center place-items-center gap-5 p-12 rounded-2xl"
                onSubmit={createSouvenir}>
                <h1 className="font-bold text-2xl">Add New Souvenir</h1>
                <Input type="text" placeholder="Souvenir Name" name="name" value={formData.name} onChange={handleInputChange} />

                <div className="grid w-full max-w-sm items-center gap-1.5">
                    <Label htmlFor="description">Description</Label>
                    <textarea
                        id="description"
                        name="description"
                        placeholder="Enter souvenir description"
                        value={formData.description}
                        onChange={handleInputChange}
                        className="border rounded-md p-2 w-full h-24 resize-none"
                    />
                </div>

                <div className="grid w-full max-w-sm items-center gap-1.5">
                    <Label htmlFor="price">Price</Label>
                    <Input type="number" name="price" id="price" value={formData.price} onChange={handleInputChange} />
                </div>

                <Select onValueChange={(value) => setFormData({ ...formData, storeId: parseInt(value) })}>
                    <SelectTrigger>
                        <SelectValue placeholder="Select Store" />
                    </SelectTrigger>
                    <SelectContent>
                        {stores.length > 0 &&
                            stores.map((store: Store) => (
                                <SelectItem key={store.id} value={store.id.toString()}>
                                    {store.name}
                                </SelectItem>
                            ))}
                    </SelectContent>
                </Select>

                <div className="grid w-full max-w-sm items-center gap-1.5">
                    <Label htmlFor="picture">Picture</Label>
                    <Input type="file" id="picture" accept="image/*" onChange={handleFileChange} />
                </div>

                <Button className="bg-purple-500 w-full">Add Souvenir</Button>
            </form>
        </div>
    );
}

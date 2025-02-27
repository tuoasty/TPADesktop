import {Input} from "@/components/ui/input.tsx";
import {Label} from "@/components/ui/label.tsx";
import {Button} from "@/components/ui/button.tsx";
import React, {useEffect, useState} from "react";
import {Restaurant} from "@/ types/restaurant.ts";
import {invoke} from "@tauri-apps/api/core";
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from "@/components/ui/select.tsx";
import {toast} from "sonner";

interface MenuFormData {
    name: string;
    price: number;
    restaurantId: number;
    imageData:string,
    imageName:string,
    mimeType:string
}

export default function AddNewMenu() {
    const [restaurants, setRestaurants] = useState<Restaurant[]>([]);
    const [formData, setFormData] = useState<MenuFormData>({
        name: "",
        price: 0,
        restaurantId: 0,
        imageData: "",
        imageName:"",
        mimeType:""
    });

    useEffect(() => {
        invoke<Restaurant[]>("find_all_restaurant")
            .then(setRestaurants)
    }, []);

    const getMimeType = (fileName: string): string => {
        const extension = fileName.split('.').pop()?.toLowerCase();
        switch (extension) {
            case 'jpg':
            case 'jpeg':
                return 'image/jpeg';
            case 'png':
                return 'image/png';
            case 'gif':
                return 'image/gif';
            case 'webp':
                return 'image/webp';
            default:
                return 'application/octet-stream';
        }
    };

    async function handleFileChange(e: React.ChangeEvent<HTMLInputElement>){
        const file = e.target.files?.[0];
        if (!file) return;

        const arrayBuffer = await file.arrayBuffer();
        const uint8Array = new Uint8Array(arrayBuffer);

        const base64String = btoa(
            Array.from(uint8Array)
                .map(byte => String.fromCharCode(byte))
                .join('')
        );

        setFormData({
            ...formData,
            imageData:base64String,
            imageName:file.name,
            mimeType:getMimeType(file.name)
        });
    }

    function handleInputChange(e: React.ChangeEvent<HTMLInputElement>) {
        setFormData({...formData, [e.target.name]: e.target.value});
    }

    async function createMenu(e:React.FormEvent) {
        e.preventDefault();

        const menuData = {
            name:formData.name,
            restaurant_id:formData.restaurantId,
            price:Number(formData.price),
            image_data:formData.imageData,
            mime_type:formData.mimeType,
            image_name:formData.imageName,

        };

        try {
            await invoke("create_menu", {menu:menuData});
            toast.success("Successfully inserted new menu")
        } catch (error) {
            toast.error(`${error}`)
        }
    }

    return (
        <div className="h-full w-full bg-purple-200 flex justify-center place-items-center">
            <form
                className="bg-white h-auto w-[25%] flex flex-col justify-center place-items-center gap-5 p-12 rounded-2xl"
                onSubmit={createMenu}>
                <h1 className="font-bold text-2xl">Add New Menu</h1>
                <Input type="text" placeholder="Menu Name" name="name" value={formData.name}
                       onChange={handleInputChange}/>
                <Input type="number" placeholder="Menu Price" name="price" value={formData.price}
                       onChange={handleInputChange}/>
                <Select onValueChange={(value) => setFormData({...formData, restaurantId:parseInt(value)})}>
                    <SelectTrigger>
                        <SelectValue placeholder="aceh"/>
                    </SelectTrigger>
                    <SelectContent>
                        {restaurants.length > 0 && (
                            restaurants.map((restaurant: Restaurant) => (
                                <SelectItem key={restaurant.id} value={restaurant.id.toString()}>{restaurant.name}</SelectItem>
                            ))
                        )}
                    </SelectContent>
                </Select>
                <div className="grid w-full max-w-sm items-center gap-1.5">
                    <Label htmlFor="picture">Picture</Label>
                    <Input type="file" id="picture" accept="image/*" onChange={handleFileChange}/>
                </div>
                <Button className="bg-purple-500 w-full">Add Menu</Button>
            </form>
        </div>
    )
}
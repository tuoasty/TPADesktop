import {LostItem, UpdateLostItemPayload} from "@/ types/lost_item.ts";
import {useForm, Controller} from "react-hook-form";
import React, {useEffect} from "react";
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger
} from "@/components/ui/dialog.tsx";
import {Button} from "@/components/ui/button.tsx";
import {Input} from "@/components/ui/input.tsx";
import {Label} from "@/components/ui/label.tsx";
import {NewImage} from "@/ types/image.ts";
import {createImageFile} from "@/lib/fileFunctions.ts";
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from "@/components/ui/select.tsx";

interface Props {
    item: LostItem,
    onUpdate: (id: number, data: UpdateLostItemPayload) => void;
}

export const UpdateItemDialog: React.FC<Props> = ({
                                                      item,
                                                      onUpdate
                                                  }) => {
    const {register, handleSubmit, reset, setValue, control, watch} = useForm<UpdateLostItemPayload>({
        defaultValues: {
            name: item.name,
            color: item.color,
            item_type: item.item_type,
            last_location: item.last_location,
            owner_id: item.owner_id,
            status: item.status,
            finder_id: item.finder_id || 0,
            found_location: item.found_location || "",
        }
    });

    const item_status = watch("status");

    useEffect(() => {
        reset({
            name: item.name,
            color: item.color,
            item_type: item.item_type,
            last_location: item.last_location,
            owner_id: item.owner_id,
            status: item.status,
            finder_id: item.finder_id || 0,
            found_location: item.found_location || "",
        });
    }, [item, reset]);

    async function handleFileChange(e: React.ChangeEvent<HTMLInputElement>) {
        const image: NewImage | undefined = await createImageFile(e);

        if (!image) {
            return
        }

        setValue("image_data", image.imageData);
        setValue("image_name", image.fileName);
        setValue("mime_type", image.mimeType);
    }

    const onSubmit = (data: UpdateLostItemPayload) => {
        onUpdate(item.id, data);
    };

    return (
        <Dialog>
            <DialogTrigger asChild>
                <Button
                    className="bg-purple-700 w-48">Update Item</Button>
            </DialogTrigger>
            <DialogContent>
                <DialogHeader>
                    <DialogTitle>Update Item Status</DialogTitle>
                    <DialogDescription>Fill the required details</DialogDescription>
                </DialogHeader>
                <form onSubmit={handleSubmit(onSubmit)}>
                    <div className="flex flex-col gap-2">
                        <Label htmlFor="name">Name</Label>
                        <Input id="name" {...register("name")}
                               placeholder="Name"
                               defaultValue={item.name}/>
                    </div>
                    <div className="flex flex-col gap-2">
                        <Label htmlFor="type">Type</Label>
                        <Input id="type" {...register("item_type")}
                               placeholder="Item Type"
                               defaultValue={item.color}/>
                    </div>
                    <div className="flex flex-col gap-2">
                        <Label htmlFor="color">Color</Label>
                        <Input id="color" {...register("color")}
                               placeholder="Color"
                               defaultValue={item.item_type}/>
                    </div>
                    <div className="flex flex-col gap-2">
                        <Label htmlFor="last_location">Last Seen Location</Label>
                        <Input id="last_location" {...register("last_location")}
                               placeholder="Last Seen Location"
                               defaultValue={item.last_location}/>
                    </div>
                    <div className="flex flex-col gap-2">
                        <Label htmlFor="owner_id">Owner ID</Label>
                        <Input id="owner_id" {...register("owner_id")}
                               placeholder="Owner ID"
                               defaultValue={item.owner_id}/>
                    </div>
                    <Label htmlFor="status">Status</Label>
                    <Controller name="status" control={control} render={
                        ({field}) => (
                            <Select onValueChange={field.onChange} value={field.value}>
                                <SelectTrigger>
                                    <SelectValue placeholder="Restaurant"/>
                                </SelectTrigger>
                                <SelectContent>
                                    <SelectItem value="Missing">Missing</SelectItem>
                                    <SelectItem value="Found">Found</SelectItem>
                                    <SelectItem value="Returned to Owner">Returned to Owner</SelectItem>
                                </SelectContent>
                            </Select>
                        )
                    }/>

                    {(item_status === "Found" || item_status === "Returned to owner") && (
                        <div className="flex flex-col gap-2">
                            <div className="flex flex-col gap-2">
                                <Label htmlFor="finder_id">Finder ID</Label>
                                <Input id="finder_id" {...register("finder_id")}
                                       placeholder="Finder ID"/>
                            </div>
                            <div className="flex flex-col gap-2">
                                <Label htmlFor="found_location">Found Location</Label>
                                <Input id="found_location" {...register("found_location")}
                                       placeholder="Found Location"/>
                            </div>
                            <div className="flex flex-col gap-2">
                                <Label htmlFor="picture">Picture</Label>
                                <Input type="file" id="picture" accept="image/*" onChange={handleFileChange}/>
                            </div>
                        </div>
                    )}
                    <DialogFooter className="mt-2">
                        <Button type="submit" className="bg-purple-700">Save changes</Button>
                    </DialogFooter>
                </form>
            </DialogContent>
        </Dialog>
    )
}
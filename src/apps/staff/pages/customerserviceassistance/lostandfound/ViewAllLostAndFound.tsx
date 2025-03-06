import {useEffect, useState} from "react";
import {LostItem, UpdateLostItemPayload} from "@/ types/lost_item.ts";
import {invoke} from "@tauri-apps/api/core";
import {toast} from "sonner";
import {UpdateItemDialog} from "@/apps/staff/pages/customerserviceassistance/lostandfound/UpdateItemDialog.tsx";


export default function ViewAllLostAndFound() {
    const [lostItem, setLostItem] = useState<LostItem[]>([]);

    const fetchLostItem = async () => {
        try {
            await invoke<LostItem[]>("find_all_lost_item").then(setLostItem)
        } catch (e) {
            toast.error(`${e}`)
        }
    }

    useEffect(() => {
        fetchLostItem();
    }, []);

    const updateItemDetail = async (item:UpdateLostItemPayload) => {
        try {
            console.log(item)
            await invoke("update_lost_item", {item:item})
            toast.success("Successfully updated item")
            fetchLostItem();
        } catch (e) {
            toast.error(`${e}`)
        }
    }

    return (
        <div className="h-screen w-full bg-purple-200 flex flex-col p-16 overflow-auto gap-5">
            {lostItem.length > 0 && (
                lostItem.map((item:LostItem) => (
                    <div key={item.id} className="w-full bg-white h-auto rounded-2xl shrink-0 flex">
                        <div className="w-96 h-auto p-8 overflow-hidden">
                            {item.status == "Missing" || !item.image_data ? (
                                <div className="flex justify-center place-items-center h-full">
                                    <h1>No Image</h1>
                                </div>
                            ) : (
                                <img className="object-contain w-full h-full rounded-lg" src={item.image_data}
                                 alt={item.name}/>
                            )}
                        </div>
                        <div className="w-full h-auto flex flex-col p-8 gap-5">
                            <div className="w-full h-full justify-between flex flex-row">
                                <div className="w-auto flex flex-col gap-2">
                                    <h1 className="font-bold text-4xl">{item.name}</h1>
                                    <h2>Type : {item.item_type}</h2>
                                    <h2>Color : {item.color}</h2>
                                    <h2>Last Location : {item.last_location}</h2>
                                    <h2>Owner : {item.owner_name}</h2>
                                    <br/>
                                    {item.status != "Missing" && (
                                        <div className="w-auto flex flex-col gap-2">
                                            <h2>Finder : {item.finder_name}</h2>
                                            <h2>Found Location : {item.found_location}</h2>
                                        </div>
                                    )}
                                </div>
                                <div className="w-48 flex justify-center place-items-center mr-6 flex-col gap-4">
                                    <h2
                                        className={`${item.status == "Found" ? "bg-yellow-400" : item.status == "Missing" ? "bg-red-500" : "bg-green-500"}
                                        rounded-lg p-2 font-bold w-full text-center`}>
                                        {item.status}
                                    </h2>
                                    <UpdateItemDialog item={item} onUpdate={updateItemDetail}/>
                                </div>
                            </div>
                        </div>
                    </div>
                    )
                )
            )}
        </div>
    )
}
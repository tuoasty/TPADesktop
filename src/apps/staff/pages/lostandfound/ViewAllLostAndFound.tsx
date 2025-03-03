import {useEffect, useState} from "react";
import {LostItem} from "@/ types/lost_item.ts";
import {invoke} from "@tauri-apps/api/core";
import {toast} from "sonner";

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
                                    <h3>Status : {item.status}</h3>
                                </div>
                                <div className="w-48 flex justify-center place-items-center mr-6 flex-col gap-4">

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
import {useStaffAuth} from "@/context/StaffAuthProvider.tsx";
import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {toast} from "sonner";
import {Staff} from "@/ types/staff.ts";
import {Store} from "@/ types/store.ts";

export default function ViewStore() {
    const {staffId} = useStaffAuth();
    const [store, setStore] = useState<Store | null>(null);

    const findStaffStore = async () => {
        try {
            invoke<Store>("find_staff_store", {selectedId: staffId}).then(setStore);
        } catch (e) {
            toast.error(`${e}`);
        }
    };

    useEffect(() => {
        findStaffStore()
    }, []);


    return (
        <div className="h-screen w-full bg-purple-200 flex flex-col p-16 overflow-auto gap-5">
            {store ? (
                <div className="w-full bg-white h-auto rounded-2xl shrink-0 flex">
                    <div className="w-96 h-auto p-8 overflow-hidden">
                        <img className="object-contain w-full h-full rounded-lg" src={store.image_data} alt={store.name}/>
                    </div>
                    <div className="w-full h-auto flex flex-col p-8 gap-5">
                        <div className="w-full h-full justify-between flex flex-row">
                            <div className="w-auto flex flex-col gap-2">
                                <h1 className="font-bold text-4xl">{store.name}</h1>
                                <h4>{store.open_time} - {store.close_time}</h4>
                                <h3>Status : {store.status}</h3>
                            </div>
                        </div>
                        <div>
                            <h1 className="font-bold text-2xl">Staffs</h1>
                            {store.staffs.length === 0 ? (
                                <h2>None</h2>
                            ) : store.staffs.map((staff: Staff) => (
                                <div key={staff.id}>
                                    <h2>{staff.name}</h2>
                                </div>
                            ))}
                        </div>
                    </div>
                </div>
            ) : (
                <h1>You are not assigned to any Store</h1>
            )}
        </div>
    );
}
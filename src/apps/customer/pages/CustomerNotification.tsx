import {useEffect, useState} from "react";
import {useCustomerAuth} from "@/context/CustomerAuthProvider.tsx";
import {invoke} from "@tauri-apps/api/core";
import {toast} from "sonner";
import {Notification} from "@/ types/notification.ts";

export default function CustomerNotification(){
    const [notifications, setNotifications] = useState<Notification[]>([])
    const {customerId} = useCustomerAuth();

    const fetchAllNotifications = async () => {
        try {
            invoke<Notification[]>("find_customer_notifications", {selectedId:customerId}).then(setNotifications)
        } catch (e) {
            toast.error(`${e}`)
        }
    }

    useEffect(() => {
        fetchAllNotifications();
    }, []);

    return (
        <div className="h-screen w-full bg-blue-300 flex flex-col p-16 overflow-auto gap-5">
            {notifications.length > 0 && (
                notifications.map((notification:Notification) => (
                    <div key={notification.id} className="w-full bg-white h-min-72 rounded-2xl shrink-0 flex justify-between">
                        <div className="w-auto h-full flex flex-col p-8 gap-2">
                            <h1 className="font-bold text-4xl">{notification.message}</h1>
                        </div>
                    </div>
                ))
            )}
        </div>
    )
}
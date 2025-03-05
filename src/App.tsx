import {useEffect, useState} from "react";
import CustomerApp from "@/apps/customer/CustomerApp.tsx";
import {invoke} from "@tauri-apps/api/core";
import StoreApp from "@/apps/store/StoreApp.tsx";
import StaffApp from "@/apps/staff/StaffApp.tsx";
import RideApp from "@/apps/ride/RideApp.tsx";
import RestaurantApp from "@/apps/restaurant/RestaurantApp.tsx";

export default function App() {
    const [appId, setAppId] = useState("1");
    useEffect(() => {
        const fetchAppId = async () => {
            try {
                const id: string = await invoke("get_app_id");
                console.log(id);
                setAppId(id);
            } catch {
                setAppId("2")
            }
        }

        fetchAppId();
    }, []);

    switch (appId) {
        case "1":
            return <CustomerApp/>
        case "2":
            return <StaffApp/>
        case "3.1":
            return <StoreApp storeId={1}/>
        case "3.2":
            return <StoreApp storeId={2}/>
        case "3.3":
            return <StoreApp storeId={3}/>
        case "4.1":
            return <RideApp rideId={1}/>
        case "4.2":
            return <RideApp rideId={2}/>
        case "4.3":
            return <RideApp rideId={3}/>
        case "5.1":
            return <RestaurantApp restaurantId={1}/>
        case "5.2":
            return <RestaurantApp restaurantId={2}/>
        case "5.3":
            return <RestaurantApp restaurantId={3}/>
        default:
            return <CustomerApp/>
    }
}
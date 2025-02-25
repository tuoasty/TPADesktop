import {useEffect, useState} from "react";
import CustomerApp from "@/apps/customer/CustomerApp.tsx";
import {invoke} from "@tauri-apps/api/core";
import StaffApp from "@/apps/StaffApp.tsx";
export default function App(){
    const [appId, setAppId] = useState<string>("customer");

    useEffect(() => {
        const fetchAppId = async ()=> {
            try {
                const id:string = await invoke("get_app_id");
                setAppId(id);
            } catch {
                let debugId = "2";
                setAppId(debugId);
                // 1 = customer
                // 2 = staff
                // 3 = ride
                // 4 = restaurant
                // 5 = store
            }
        }

        fetchAppId();
    }, []);

    switch(appId){
        case "1":
            return <CustomerApp/>
        case "2":
            return <StaffApp/>
        default:
            return <CustomerApp/>
    }
}
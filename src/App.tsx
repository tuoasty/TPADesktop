import {useEffect, useState} from "react";
import CustomerApp from "@/apps/CustomerApp.tsx";
import {invoke} from "@tauri-apps/api/core";
import "./main.css";
import StaffApp from "@/apps/StaffApp.tsx";

export default function App(){
    const [appId, setAppId] = useState<string>("1");

    useEffect(() => {
        const fetchAppId = async ()=> {
            try {
                const id:string = await invoke("get_app_id");
                setAppId(id);
            } catch {
                setAppId("2");

                // 1 = Customer
                // 2 = Staff
                // 3 = Ride
                // 4 = Restaurant
                // 5 = Store
            }
        }

        fetchAppId();
    }, []);

    switch(appId){
        case `1`:
            return <CustomerApp/>
        case `2`:
            return <StaffApp/>
        default:
            return <CustomerApp/>
    }
}
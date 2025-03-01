import {useEffect, useState} from "react";
import CustomerApp from "@/apps/customer/CustomerApp.tsx";
import {invoke} from "@tauri-apps/api/core";
import StaffApp from "@/apps/StaffApp.tsx";

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
        default:
            return <CustomerApp/>
    }
}
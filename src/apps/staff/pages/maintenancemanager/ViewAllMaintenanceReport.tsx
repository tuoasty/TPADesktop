import {useEffect, useState} from "react";
import {MaintenanceReport} from "@/ types/maintenance_report.ts";
import {invoke} from "@tauri-apps/api/core";

export default function ViewAllMaintenanceReport() {
    const [maintenanceReports, setMaintenanceReports] = useState<MaintenanceReport[]>([]);

    const fetchMaintenanceReports = async () => {
        invoke<MaintenanceReport[]>("find_all_maintenance_report").then(setMaintenanceReports)
    }

    useEffect(() => {
        fetchMaintenanceReports();
    }, []);

    return (
        <div className="h-screen w-full bg-purple-200 flex flex-col p-16 overflow-auto gap-5">
            {maintenanceReports.length > 0 && (
                maintenanceReports.map((report:MaintenanceReport)=> (
                    <div key={report.id} className="w-full bg-white h-min-72 rounded-2xl shrink-0 flex">
                        <div className="w-full h-full flex flex-col p-8 gap-2">
                            <h1 className="font-bold text-4xl">{report.ride_name}</h1>
                            <h2 className="text-2xl">Status : {report.status}</h2>
                            <h4>Description : {report.description}</h4>
                        </div>
                    </div>
                ))
            )}
        </div>
    )
}
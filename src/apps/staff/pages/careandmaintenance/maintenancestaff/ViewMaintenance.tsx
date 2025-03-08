import {useStaffAuth} from "@/context/StaffAuthProvider.tsx";
import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {toast} from "sonner";
import {MaintenanceReport} from "@/ types/maintenance_report.ts";
import {Button} from "@/components/ui/button.tsx";

export default function ViewMaintenance() {
    const {staffId} = useStaffAuth();
    const [maintenance, setMaintenance] = useState<MaintenanceReport | null>(null);

    const fetchStaffMaintenance = async () => {
        try {
            invoke<MaintenanceReport>("find_staff_maintenance", {selectedId: staffId}).then(setMaintenance);
        } catch (e) {
            toast.error(`${e}`);
        }
    };

    const submitTask = async () => {
        try {
            invoke("submit_task", {selectedMaintenanceId:maintenance?.id})
            toast.success("Successfully submitted task")
            fetchStaffMaintenance();
        } catch (e) {
            toast.error(`${e}`)
        }
    };

    useEffect(() => {
        fetchStaffMaintenance();
    }, []);

    return (
        <div className="h-screen w-full bg-purple-200 flex justify-center items-center p-8">
            {maintenance ? (
                <div className="w-3/5 bg-white h-auto rounded-2xl flex flex-col p-8 gap-5 shadow-lg">
                    <div className="w-full flex flex-col items-center gap-5">
                        <h1 className="font-bold text-4xl text-center">{maintenance.ride_name}</h1>
                        <h3 className="text-lg">Status: {maintenance.status}</h3>
                        <h4 className="text-md">Maintenance Report ID: {maintenance.id}</h4>
                        <div className="text-center">
                            <h1 className="font-bold text-2xl">Maintenance Details</h1>
                            <p><strong>Description:</strong> {maintenance.description}</p>
                            <p><strong>Staff Assigned:</strong> {maintenance.staff_name ? maintenance.staff_name : "Not Assigned"}</p>
                        </div>
                        <Button
                            className="mt-4 px-6 py-2 bg-purple-700 text-white"
                            onClick={submitTask}
                            disabled={maintenance.status == "Pending Review" || maintenance.status == "Finished"}
                        >
                            Finish Maintenance
                        </Button>
                    </div>
                </div>
            ) : (
                <h1>You are not assigned to any maintenance</h1>
            )}
        </div>
    );
}

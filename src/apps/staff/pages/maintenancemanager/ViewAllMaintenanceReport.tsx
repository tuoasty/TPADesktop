import {useEffect, useState} from "react";
import {MaintenanceReport} from "@/ types/maintenance_report.ts";
import {invoke} from "@tauri-apps/api/core";
import {Button} from "@/components/ui/button.tsx";
import {
    AlertDialog,
    AlertDialogAction,
    AlertDialogCancel,
    AlertDialogContent, AlertDialogDescription, AlertDialogFooter,
    AlertDialogHeader,
    AlertDialogTitle,
    AlertDialogTrigger
} from "@/components/ui/alert-dialog.tsx";
import {Staff} from "@/ types/staff.ts";
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from "@/components/ui/select.tsx";
import {toast} from "sonner";
export default function ViewAllMaintenanceReport() {
    const [maintenanceReports, setMaintenanceReports] = useState<MaintenanceReport[]>([]);
    const [maintenanceStaff, setMaintenanceStaffs] = useState<Staff[]>([]);
    const [selectedId, setSelectedId] = useState<number | null>(null);

    const fetchMaintenanceReports = async () => {
        invoke<MaintenanceReport[]>("find_all_maintenance_report").then(setMaintenanceReports)
    }

    const fetchMaintenanceStaffs = async() => {
        invoke<Staff[]>("find_all_maintenance_staff").then(setMaintenanceStaffs)
    }

    useEffect(() => {
        fetchMaintenanceReports();
        fetchMaintenanceStaffs();
    }, []);

    const acceptRequest = async () => {
        if(selectedId == null){
            toast.error("Please select a staff")
        } else {

        }
    }

    const rejectRequest = async () => {

    }

    return (
        <div className="h-screen w-full bg-purple-200 flex flex-col p-16 overflow-auto gap-5">
            {maintenanceReports.length > 0 && (
                maintenanceReports.map((report:MaintenanceReport)=> (
                    <div key={report.id} className="w-full bg-white h-min-72 rounded-2xl shrink-0 flex justify-between">
                        <div className="w-auto h-full flex flex-col p-8 gap-2">
                            <h1 className="font-bold text-4xl">{report.ride_name}</h1>
                            <h2 className="text-2xl">Status : {report.status}</h2>
                            {report.status == "In Progress" && (
                                <h3>Assigned Staff : {report.staff_name}</h3>
                            )}
                            <h4>Description : {report.description}</h4>
                        </div>
                        <div className="flex flex-col gap-5 justify-center pr-8">
                            <AlertDialog>
                                <AlertDialogTrigger disabled={report.status == "Completed" || report.status == "Rejected" || report.status == "In Progress"}>
                                    <Button className="w-48 h-12 bg-green-500"
                                            disabled={report.status == "Completed" || report.status == "Rejected" || report.status == "In Progress"}>
                                        Accept Report
                                    </Button>
                                </AlertDialogTrigger>
                                <AlertDialogContent>
                                    <AlertDialogHeader>
                                        <AlertDialogTitle>Choose Staff to Assign</AlertDialogTitle>
                                        <AlertDialogDescription>
                                            Make sure the staff is free
                                        </AlertDialogDescription>
                                    </AlertDialogHeader>
                                    <Select onValueChange={(val) => setSelectedId(Number(val))}>
                                        <SelectTrigger>
                                            <SelectValue placeholder="Maintenance Staff"/>
                                        </SelectTrigger>
                                        <SelectContent>
                                            {maintenanceStaff.length > 0 && (
                                                maintenanceStaff.map((staff: Staff) => (
                                                    <SelectItem key={staff.id} value={staff.id.toString()}>{staff.name}</SelectItem>
                                                ))
                                            )}
                                        </SelectContent>
                                    </Select>
                                    <AlertDialogFooter>
                                        <AlertDialogCancel>Cancel</AlertDialogCancel>
                                        <AlertDialogAction
                                            onClick={acceptRequest}>Confirm</AlertDialogAction>
                                    </AlertDialogFooter>
                                </AlertDialogContent>
                            </AlertDialog>
                            <Button onClick={rejectRequest} className="w-48 h-12 bg-red-500"
                                    disabled={report.status == "Completed" || report.status == "Rejected" || report.status == "In Progress"}>
                                Decline Report
                            </Button>
                        </div>
                    </div>
                ))
            )}
        </div>
    )
}
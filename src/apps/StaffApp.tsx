import {BrowserRouter, Navigate, Route, Routes} from "react-router-dom";
import {StaffAuthProvider, ProtectedRoute} from "@/context/StaffAuthProvider.tsx";
import StaffLogin from "@/apps/staff/pages/StaffLogin.tsx";
import CreateStaffAccount from "@/apps/staff/pages/coo/CreateStaffAccount.tsx";
import StaffDashboard from "@/apps/staff/pages/StaffDashboard.tsx";
import StaffNavbar from "@/apps/staff/components/navbar/StaffNavbar.tsx";
import {Toaster} from "@/components/ui/sonner.tsx";
import AddNewMenu from "@/apps/staff/pages/fbsupervisor/AddNewMenu.tsx";
import ViewAllRestaurant from "@/apps/staff/pages/fbsupervisor/ViewAllRestaurant.tsx";
import ViewAllStore from "@/apps/staff/pages/retailmanager/ViewAllStore.tsx";

export default function StaffApp() {
    return (
        <BrowserRouter>
            <StaffAuthProvider>
                <StaffNavbar/>
                <div className="pt-20 h-screen flex flex-col w-full overflow-auto">
                    <Routes>
                        <Route path="*" element={<Navigate to="/staff/login" replace/>}/>
                        <Route path="/staff">
                            <Route path="login" element={<StaffLogin/>}/>
                            <Route path="create-account" element={<ProtectedRoute
                                allowedRoles={["COO"]}><CreateStaffAccount/></ProtectedRoute>}/>
                            <Route path="add-new-menu" element={<ProtectedRoute
                                allowedRoles={["F&B Supervisor"]}><AddNewMenu/></ProtectedRoute>}/>
                            <Route path="view-all-restaurant" element={<ProtectedRoute
                                allowedRoles={["F&B Supervisor"]}><ViewAllRestaurant/></ProtectedRoute>}/>
                            <Route path="view-all-store" element={<ProtectedRoute
                                allowedRoles={["Retail Manager"]}><ViewAllStore/></ProtectedRoute>}/>
                            <Route path="/staff" element={<ProtectedRoute><StaffDashboard/></ProtectedRoute>}/>
                        </Route>
                    </Routes>
                </div>
            </StaffAuthProvider>
            <Toaster/>
        </BrowserRouter>
    )
}
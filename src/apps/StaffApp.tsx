import {BrowserRouter, Route, Routes} from "react-router-dom";
import {StaffAuthProvider, ProtectedRoute} from "@/context/StaffAuthProvider.tsx";
import StaffLogin from "@/apps/staff/pages/StaffLogin.tsx";
import CreateStaffAccount from "@/apps/staff/pages/CreateStaffAccount.tsx";
import StaffDashboard from "@/apps/staff/pages/StaffDashboard.tsx";
import StaffNavbar from "@/apps/staff/components/navbar/StaffNavbar.tsx";

export default function StaffApp() {
    return (
            <BrowserRouter>
                <StaffAuthProvider>
                    <StaffNavbar/>
                    <div className="pt-20 h-screen w-screen">
                        <Routes>
                            <Route path="/staff">
                                <Route path="login" element={<StaffLogin/>}/>
                                <Route path="create-account" element={
                                    <ProtectedRoute allowedRoles={["COO"]}>
                                        <CreateStaffAccount/>
                                    </ProtectedRoute>
                                }/>
                                <Route path="*" element={
                                    <ProtectedRoute>
                                        <StaffDashboard/>
                                    </ProtectedRoute>
                                }/>
                            </Route>
                        </Routes>
                    </div>
                </StaffAuthProvider>
            </BrowserRouter>
    )
}
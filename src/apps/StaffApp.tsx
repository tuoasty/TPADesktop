import {BrowserRouter, Route, Routes} from "react-router-dom";
import {StaffAuthProvider, ProtectedRoute} from "@/auth/StaffAuthProvider.tsx";
import StaffLogin from "@/pages/staff/StaffLogin.tsx";
import StaffRegister from "@/pages/staff/StaffRegister.tsx";
import StaffDashboard from "@/pages/staff/StaffDashboard.tsx";

export default function StaffApp(){
    return (
        <BrowserRouter>
            <StaffAuthProvider>
                <Routes>
                    <Route path="/login" element={<StaffLogin/>} />
                    <Route path="/register" element={<StaffRegister/>}/>
                    <Route path="/*" element={
                        <ProtectedRoute>
                            <StaffDashboard/>
                        </ProtectedRoute>
                    }/>
                </Routes>
            </StaffAuthProvider>
        </BrowserRouter>
    )
}
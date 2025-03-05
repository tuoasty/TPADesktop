import {BrowserRouter, Navigate, Route, Routes} from "react-router-dom";
import {CustomerAuthProvider} from "@/context/CustomerAuthProvider.tsx";
import {Toaster} from "@/components/ui/sonner.tsx";
import RideNavbar from "@/apps/ride/RideNavbar.tsx";
import RideLogin from "@/apps/ride/RideLogin.tsx";
import RideDashboard from "@/apps/ride/RideDashboard.tsx";

interface Props {
    rideId:number;
}
function RideApp(p:Props) {
    return (
        <BrowserRouter>
            <CustomerAuthProvider>
                <RideNavbar/>
                <div className="pt-20 h-screen flex flex-col w-full overflow-auto">
                    <Routes>
                        <Route path="*" element={<Navigate to="/ride" replace/>}/>
                        <Route path="/ride">
                            <Route path="login" element={<RideLogin/>}/>
                            <Route path="/ride" element={<RideDashboard rideId={p.rideId}/>}/>
                        </Route>
                    </Routes>
                </div>
            </CustomerAuthProvider>
            <Toaster/>
        </BrowserRouter>
    );
}

export default RideApp;

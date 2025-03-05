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
import ViewAllRide from "@/apps/staff/pages/ridemanager/ViewAllRide.tsx";
import ViewAllMaintenanceReport from "@/apps/staff/pages/maintenancemanager/ViewAllMaintenanceReport.tsx";
import ViewAllLostAndFound from "@/apps/staff/pages/lostandfound/ViewAllLostAndFound.tsx";
import ViewRideProposal from "@/apps/staff/pages/coo/ViewRideProposal.tsx";
import ViewRestaurantProposal from "@/apps/staff/pages/cfo/ViewRestaurantProposal.tsx";
import ViewStoreProposal from "@/apps/staff/pages/ceo/ViewStoreProposal.tsx";
import BroadcastMessage from "@/apps/staff/pages/customerservice/BroadcastMessage.tsx";
import CreateCustomerAccount from "@/apps/staff/pages/customerservice/CreateCustomerAccount.tsx";
import GlobalChat from "@/apps/staff/GlobalChat.tsx";
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
                                    allowedRoles={["F&B Supervisor", "Customer Service"]}><ViewAllRestaurant/></ProtectedRoute>}/>
                                <Route path="view-all-store" element={<ProtectedRoute
                                    allowedRoles={["Retail Manager", "COO"]}><ViewAllStore/></ProtectedRoute>}/>
                                <Route path="view-all-ride" element={<ProtectedRoute
                                    allowedRoles={["Ride Manager", "COO", "Customer Service"]}><ViewAllRide/></ProtectedRoute>}/>
                                <Route path="view-all-maintenance-report" element={<ProtectedRoute
                                    allowedRoles={["Maintenance Manager", "COO"]}><ViewAllMaintenanceReport/></ProtectedRoute>}/>
                                <Route path="view-lost-and-found" element={<ProtectedRoute
                                    allowedRoles={["Lost and Found Staff"]}><ViewAllLostAndFound/></ProtectedRoute>}/>
                                <Route path="view-ride-proposal" element={<ProtectedRoute
                                    allowedRoles={["COO"]}><ViewRideProposal/></ProtectedRoute>}/>
                                <Route path="view-restaurant-proposal" element={<ProtectedRoute
                                    allowedRoles={["CEO", "CFO"]}><ViewRestaurantProposal/></ProtectedRoute>}/>
                                <Route path="view-store-proposal" element={<ProtectedRoute
                                    allowedRoles={["CEO"]}><ViewStoreProposal/></ProtectedRoute>}/>
                                <Route path="send-broadcast-message" element={<ProtectedRoute
                                    allowedRoles={["Customer Service"]}><BroadcastMessage/></ProtectedRoute>}/>
                                <Route path="create-customer-account" element={<ProtectedRoute
                                    allowedRoles={["Customer Service"]}><CreateCustomerAccount/></ProtectedRoute>}/>
                                <Route path="staff-chat" element={<ProtectedRoute><GlobalChat/></ProtectedRoute>}/>
                                <Route path="/staff" element={<ProtectedRoute><StaffDashboard/></ProtectedRoute>}/>
                            </Route>
                        </Routes>
                    </div>
            </StaffAuthProvider>
            <Toaster/>
        </BrowserRouter>
    )
}
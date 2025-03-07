import {BrowserRouter, Navigate, Route, Routes} from "react-router-dom";
import {StaffAuthProvider, ProtectedRoute} from "@/context/StaffAuthProvider.tsx";
import StaffLogin from "@/apps/staff/pages/StaffLogin.tsx";
import StaffDashboard from "@/apps/staff/pages/StaffDashboard.tsx";
import StaffNavbar from "@/apps/staff/components/navbar/StaffNavbar.tsx";
import {Toaster} from "@/components/ui/sonner.tsx";

import GlobalChat from "@/apps/staff/GlobalChat.tsx";
import AddNewMenu from "@/apps/staff/pages/consumption/fbsupervisor/AddNewMenu.tsx";
import ViewAllRestaurant from "@/apps/staff/pages/consumption/fbsupervisor/ViewAllRestaurant.tsx";
import ViewAllStore from "@/apps/staff/pages/marketing/retailmanager/ViewAllStore.tsx";
import ViewAllRide from "@/apps/staff/pages/operational/ridemanager/ViewAllRide.tsx";
import ViewAllMaintenanceReport
    from "@/apps/staff/pages/careandmaintenance/maintenancemanager/ViewAllMaintenanceReport.tsx";
import ViewAllLostAndFound from "@/apps/staff/pages/customerserviceassistance/lostandfound/ViewAllLostAndFound.tsx";
import ViewRideProposal from "@/apps/staff/pages/executive/coo/ViewRideProposal.tsx";
import ViewRestaurantProposal from "@/apps/staff/pages/executive/cfo/ViewRestaurantProposal.tsx";
import ViewStoreProposal from "@/apps/staff/pages/executive/ceo/ViewStoreProposal.tsx";
import BroadcastMessage from "@/apps/staff/pages/customerserviceassistance/customerservice/BroadcastMessage.tsx";
import CreateCustomerAccount
    from "@/apps/staff/pages/customerserviceassistance/customerservice/CreateCustomerAccount.tsx";
import CreateStaffAccount from "@/apps/staff/pages/executive/coo/CreateStaffAccount.tsx";
import CareAndMaintenanceChat from "@/apps/staff/pages/careandmaintenance/CareAndMaintenanceChat.tsx";
import ConsumptionChat from "@/apps/staff/pages/consumption/ConsumptionChat.tsx";
import ExecutiveChat from "@/apps/staff/pages/executive/ExecutiveChat.tsx";
import MarketingChat from "@/apps/staff/pages/marketing/MarketingChat.tsx";
import OperationalChat from "@/apps/staff/pages/operational/OperationalChat.tsx";
import ViewRide from "@/apps/staff/pages/operational/ridestaff/ViewRide.tsx";
import ViewRestaurant from "@/apps/staff/pages/consumption/ViewRestaurant.tsx";
import CustomerServiceChat from "@/apps/staff/pages/customerserviceassistance/customerservice/CustomerServiceChat.tsx";
import LostAndFoundChat from "@/apps/staff/pages/customerserviceassistance/lostandfound/LostAndFoundChat.tsx";
import ChatMaintenanceAccount from "@/apps/staff/pages/operational/ridemanager/ChatMaintenanceAccount.tsx";
import MaintenanceOfficialAccount
    from "@/apps/staff/pages/careandmaintenance/maintenancemanager/MaintenanceOfficialAccount.tsx";
import ViewStore from "@/apps/staff/pages/marketing/salesassociate/ViewStore.tsx";
import ViewMaintenance from "@/apps/staff/pages/careandmaintenance/maintenancestaff/ViewMaintenance.tsx";
import ViewRideRevenue from "@/apps/staff/pages/operational/ridemanager/ViewRideRevenue.tsx";
import ViewRestaurantRevenue from "@/apps/staff/pages/consumption/fbsupervisor/ViewRestaurantRevenue.tsx";
import ViewStoreRevenue from "@/apps/staff/pages/marketing/retailmanager/ViewStoreRevenue.tsx";
import ProposeNewRestaurant from "@/apps/staff/pages/consumption/fbsupervisor/ProposeNewRestaurant.tsx";
import ProposeNewRide from "@/apps/staff/pages/operational/ridemanager/ProposeNewRide.tsx";
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
                                    allowedRoles={["Retail Manager", "CEO"]}><ViewAllStore/></ProtectedRoute>}/>
                                <Route path="view-all-ride" element={<ProtectedRoute
                                    allowedRoles={["Ride Manager", "CEO", "Customer Service"]}><ViewAllRide/></ProtectedRoute>}/>
                                <Route path="view-all-maintenance-report" element={<ProtectedRoute
                                    allowedRoles={["Maintenance Manager", "COO" , "CEO"]}><ViewAllMaintenanceReport/></ProtectedRoute>}/>
                                <Route path="view-ride" element={<ProtectedRoute
                                allowedRoles={["Ride Staff"]}><ViewRide/></ProtectedRoute>}/>
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
                                <Route path="care-and-maintenance-chat" element={<ProtectedRoute
                                    allowedRoles={["Maintenance Manager", "Maintenance Staff"]}><CareAndMaintenanceChat/></ProtectedRoute>}/>
                                <Route path="consumption-chat" element={<ProtectedRoute
                                    allowedRoles={["F&B Supervisor", "Waiter", "Chef"]}><ConsumptionChat/></ProtectedRoute>}/>
                                <Route path="executive-chat" element={<ProtectedRoute
                                    allowedRoles={["COO", "CFO", "CEO"]}><ExecutiveChat/></ProtectedRoute>}/>
                                <Route path="marketing-chat" element={<ProtectedRoute
                                    allowedRoles={["Retail Manager", "Sales Associate"]}><MarketingChat/></ProtectedRoute>}/>
                                <Route path="operational-chat" element={<ProtectedRoute
                                    allowedRoles={["Ride Manager", "Ride Staff"]}><OperationalChat/></ProtectedRoute>}/>
                                <Route path="lost-and-found-chat" element={<ProtectedRoute
                                    allowedRoles={["Lost and Found Staff"]}><LostAndFoundChat/></ProtectedRoute>}/>
                                <Route path="customer-service-chat" element={<ProtectedRoute
                                    allowedRoles={["Customer Service"]}><CustomerServiceChat/></ProtectedRoute>}/>
                                <Route path="maintenance-account-chat" element={<ProtectedRoute
                                    allowedRoles={["Ride Manager"]}><ChatMaintenanceAccount/></ProtectedRoute>}/>
                                <Route path="maintenance-official-account" element={<ProtectedRoute
                                    allowedRoles={["Maintenance Manager"]}><MaintenanceOfficialAccount/></ProtectedRoute>}/>
                                <Route path="view-restaurant" element={<ProtectedRoute
                                    allowedRoles={["Waiter", "Chef"]}><ViewRestaurant/></ProtectedRoute>}/>
                                <Route path="view-store" element={<ProtectedRoute
                                    allowedRoles={["Sales Associate"]}><ViewStore/></ProtectedRoute>}/>
                                <Route path="view-maintenance" element={<ProtectedRoute
                                    allowedRoles={["Maintenance Staff"]}><ViewMaintenance/></ProtectedRoute>}/>
                                <Route path="view-ride-revenue" element={<ProtectedRoute
                                    allowedRoles={["Ride Manager", "CFO"]}><ViewRideRevenue/></ProtectedRoute>}/>
                                <Route path="view-restaurant-revenue" element={<ProtectedRoute
                                    allowedRoles={["F&B Supervisor", "CFO"]}><ViewRestaurantRevenue/></ProtectedRoute>}/>
                                <Route path="view-store-revenue" element={<ProtectedRoute
                                    allowedRoles={["Retail Manager", "CFO"]}><ViewStoreRevenue/></ProtectedRoute>}/>
                                <Route path="propose-new-restaurant" element={<ProtectedRoute
                                    allowedRoles={["F&B Supervisor"]}><ProposeNewRestaurant/></ProtectedRoute>}/>
                                <Route path="staff-chat" element={<ProtectedRoute><GlobalChat/></ProtectedRoute>}/>
                                <Route path="/staff" element={<ProtectedRoute><StaffDashboard/></ProtectedRoute>}/>
                                <Route path="propose-new-ride" element={<ProtectedRoute
                                    allowedRoles={["Ride Manager"]}><ProposeNewRide/></ProtectedRoute>}/>
                            </Route>
                        </Routes>
                    </div>
            </StaffAuthProvider>
            <Toaster/>
        </BrowserRouter>
    )
}
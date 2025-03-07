import {BrowserRouter, Navigate, Route, Routes} from "react-router-dom";
import CustomerNavbar from "@/apps/customer/components/CustomerNavbar.tsx";
import CustomerLogin from "@/apps/customer/pages/CustomerLogin.tsx";
import CustomerDashboard from "@/apps/customer/pages/CustomerDashboard.tsx";
import {CustomerAuthProvider, ProtectedRoute} from "@/context/CustomerAuthProvider.tsx";
import CustomerNotification from "@/apps/customer/pages/CustomerNotification.tsx";
import {Toaster} from "@/components/ui/sonner.tsx";
import CustomerRide from "@/apps/customer/pages/CustomerRide.tsx";
import CustomerRestaurant from "@/apps/customer/pages/CustomerRestaurant.tsx";
import CustomerChatService from "@/apps/customer/pages/CustomerChatService.tsx";

function CustomerApp() {
  return (
      <BrowserRouter>
          <CustomerAuthProvider>
              <CustomerNavbar/>
              <div className="pt-20 h-screen flex flex-col w-full overflow-auto">
                  <Routes>
                      <Route path="*" element={<Navigate to="/customer/login" replace/>}/>
                      <Route path="/customer">
                          <Route path="login" element={<CustomerLogin/>}/>
                          <Route path="ride" element={<CustomerRide/>}/>
                          <Route path="restaurant" element={<CustomerRestaurant/>}/>
                          <Route path="notification" element={<ProtectedRoute>
                              <CustomerNotification/>
                          </ProtectedRoute>}/>
                          <Route path="customer-service" element={<ProtectedRoute>
                              <CustomerChatService/>
                          </ProtectedRoute>}/>
                          <Route path="/customer" element={<ProtectedRoute>
                              <CustomerDashboard/>
                          </ProtectedRoute>}/>

                      </Route>
                  </Routes>
              </div>
          </CustomerAuthProvider>
          <Toaster/>
      </BrowserRouter>
  );
}

export default CustomerApp;

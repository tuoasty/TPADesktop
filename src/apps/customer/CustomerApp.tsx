import {BrowserRouter, Navigate, Route, Routes} from "react-router-dom";
import CustomerNavbar from "@/apps/customer/components/CustomerNavbar.tsx";
import CustomerLogin from "@/apps/customer/pages/CustomerLogin.tsx";
import CustomerDashboard from "@/apps/customer/pages/CustomerDashboard.tsx";
import {CustomerAuthProvider, ProtectedRoute} from "@/context/CustomerAuthProvider.tsx";

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
                          <Route path="/customer" element={<ProtectedRoute>
                              <CustomerDashboard/>
                          </ProtectedRoute>}/>
                      </Route>
                  </Routes>
              </div>
          </CustomerAuthProvider>
      </BrowserRouter>
  );
}

export default CustomerApp;

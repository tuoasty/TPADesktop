import {BrowserRouter, Navigate, Route, Routes} from "react-router-dom";
import {CustomerAuthProvider} from "@/context/CustomerAuthProvider.tsx";
import {Toaster} from "@/components/ui/sonner.tsx";
import RestaurantNavbar from "@/apps/restaurant/RestaurantNavbar.tsx";
import RestaurantLogin from "@/apps/restaurant/RestaurantLogin.tsx";
import RestaurantDashboard from "@/apps/restaurant/RestaurantDashboard.tsx";

interface Props {
    restaurantId:number;
}
function RestaurantApp(p:Props) {
    return (
        <BrowserRouter>
            <CustomerAuthProvider>
                <RestaurantNavbar/>
                <div className="pt-20 h-screen flex flex-col w-full overflow-auto">
                    <Routes>
                        <Route path="*" element={<Navigate to="/restaurant" replace/>}/>
                        <Route path="/restaurant">
                            <Route path="login" element={<RestaurantLogin/>}/>
                            <Route path="/restaurant" element={<RestaurantDashboard restaurantId={p.restaurantId}/>}/>
                        </Route>
                    </Routes>
                </div>
            </CustomerAuthProvider>
            <Toaster/>
        </BrowserRouter>
    );
}

export default RestaurantApp;

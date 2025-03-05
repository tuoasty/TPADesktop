import {BrowserRouter, Navigate, Route, Routes} from "react-router-dom";
import {CustomerAuthProvider} from "@/context/CustomerAuthProvider.tsx";
import StoreLogin from "@/apps/store/StoreLogin.tsx";
import StoreDashboard from "@/apps/store/StoreDashboard.tsx";
import StoreNavbar from "@/apps/store/StoreNavbar.tsx";
import {Toaster} from "@/components/ui/sonner.tsx";

interface Props {
    storeId:number;
}
function StoreApp(p:Props) {
    return (
        <BrowserRouter>
            <CustomerAuthProvider>
                <StoreNavbar/>
                <div className="pt-20 h-screen flex flex-col w-full overflow-auto">
                    <Routes>
                        <Route path="*" element={<Navigate to="/store" replace/>}/>
                        <Route path="/store">
                            <Route path="login" element={<StoreLogin/>}/>
                            <Route path="/store" element={<StoreDashboard storeId={p.storeId}/>}/>
                        </Route>
                    </Routes>
                </div>
            </CustomerAuthProvider>
            <Toaster/>
        </BrowserRouter>
    );
}

export default StoreApp;

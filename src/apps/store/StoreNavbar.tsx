import {CustomerNavButton} from "@/apps/customer/components/CustomerNavButton.tsx";
import {CustomerLogout} from "@/apps/customer/components/CustomerLogout.tsx";

export default function StoreNavbar(){
    return (
        <div className="fixed bg-yellow-400 w-full h-20 top-0 flex flex-row">
            <CustomerNavButton text="Home" to="/store/"/>
            <CustomerLogout/>
            <CustomerNavButton text="Login" to="/store/login"/>
        </div>
    )
}
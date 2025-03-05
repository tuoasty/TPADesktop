import {CustomerNavButton} from "@/apps/customer/components/CustomerNavButton.tsx";
import {CustomerLogout} from "@/apps/customer/components/CustomerLogout.tsx";

export default function RideNavbar(){
    return (
        <div className="fixed bg-red-500 w-full h-20 top-0 flex flex-row">
            <CustomerNavButton text="Home" to="/ride/"/>
            <CustomerLogout/>
            <CustomerNavButton text="Login" to="/ride/login"/>
        </div>
    )
}
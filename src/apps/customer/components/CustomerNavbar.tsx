import {CustomerNavButton} from "@/apps/customer/components/CustomerNavButton.tsx";
import {CustomerLogout} from "@/apps/customer/components/CustomerLogout.tsx";

export default function CustomerNavbar(){
    return (
        <div className="fixed bg-blue-500 w-full h-20 top-0 flex flex-row">
            <CustomerNavButton text="Home" to="/customer/"/>
            <CustomerLogout/>
            <CustomerNavButton text="Login" to="/customer/login"/>
            <CustomerNavButton text="Notifications" to="/customer/notification"/>
            <CustomerNavButton text="Ride" to="/customer/ride"/>
            <CustomerNavButton text="Restaurant" to="/customer/restaurant"/>
        </div>
    )
}
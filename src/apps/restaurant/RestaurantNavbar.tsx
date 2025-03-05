import {CustomerNavButton} from "@/apps/customer/components/CustomerNavButton.tsx";
import {CustomerLogout} from "@/apps/customer/components/CustomerLogout.tsx";

export default function RestaurantNavbar(){
    return (
        <div className="fixed bg-green-500 w-full h-20 top-0 flex flex-row">
            <CustomerNavButton text="Home" to="/restaurant/"/>
            <CustomerLogout/>
            <CustomerNavButton text="Login" to="/restaurant/login"/>
        </div>
    )
}
import {CustomerNavButton} from "@/apps/customer/components/CustomerNavButton.tsx";

export default function CustomerNavbar(){
    return (
        <main className="fixed bg-blue-500 w-full h-20 top-0 flex flex-row">
            <CustomerNavButton text="Home" to="/customer/"/>
            <CustomerNavButton text="Login" to="/customer/login"/>
            <CustomerNavButton text="Notifications" to="/customer/notification"/>
        </main>
    )
}
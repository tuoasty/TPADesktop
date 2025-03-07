import {StaffNavButton} from "@/apps/staff/components/navbar/StaffNavButton.tsx";
import {useStaffAuth} from "@/context/StaffAuthProvider.tsx";
import {StaffLogoutMenu} from "@/apps/staff/components/navbar/StaffLogoutMenu.tsx";
import {StaffDashboardMenu} from "@/apps/staff/components/navbar/StaffDashboardMenu.tsx";

export default function StaffNavbar(){
    const {role, isAuthenticated} = useStaffAuth();

    const navItems = [
        {text:"Create Staff Account", key:1, to:"/staff/create-account", roles: ["COO"]},
        {text:"Add New Menu", key:2, to:"/staff/add-new-menu", roles: ["F&B Supervisor"]},
        {text:"View All Restaurant", key:3, to:"/staff/view-all-restaurant", roles: ["F&B Supervisor", "Customer Service"]},
        {text:"View All Store", key:4, to:"/staff/view-all-store", roles: ["Retail Manager", "CEO"]},
        {text:"View All Ride", key:5, to:"/staff/view-all-ride", roles: ["Ride Manager", "CEO", "Customer Service"]},
        {text:"View All Maintenance Report", key:6, to:"/staff/view-all-maintenance-report", roles: ["Maintenance Manager", "CEO", "COO"]},
        {text:"View All Lost Items", key:7, to:"/staff/view-lost-and-found", roles: ["Lost and Found Staff"]},
        {text:"View Ride Proposal", key:8, to:"/staff/view-ride-proposal", roles: ["COO"]},
        {text:"View Restaurant Proposal", key:9, to:"/staff/view-restaurant-proposal", roles: ["CEO", "CFO"]},
        {text:"View Store Proposal", key:10, to:"/staff/view-store-proposal", roles: ["CEO"]},
        {text:"Broadcast Message", key:11, to:"/staff/send-broadcast-message", roles: ["Customer Service"]},
        {text:"Create Customer Account", key:12, to:"/staff/create-customer-account", roles: ["Customer Service"]},
        {text:"Care and Maintenance Chat", key:13, to:"/staff/care-and-maintenance-chat", roles: ["Maintenance Manager", "Maintenance Staff", "CEO"]},
        {text:"Consumption Chat", key:14, to:"/staff/consumption-chat", roles: ["F&B Supervisor", "Waiter", "Chef"]},
        {text:"Executive Chat", key:15, to:"/staff/executive-chat", roles: ["COO", "CFO", "CEO"]},
        {text:"Marketing Chat", key:16, to:"/staff/marketing-chat", roles: ["Retail Manager", "Sales Associate", "CEO"]},
        {text:"Operational Chat", key:17, to:"/staff/operational-chat", roles: ["Ride Manager", "Ride Staff", "CEO"]},
        {text:"Lost and Found Chat", key:18, to:"/staff/lost-and-found-chat", roles: ["Lost and Found Staff"]},
        {text:"View Ride", key:19, to:"/staff/view-ride", roles: ["Ride Staff"]},
        {text:"View Restaurant", key:20, to:"/staff/view-restaurant", roles: ["Waiter", "Chef"]},
        {text:"Customer Service Chat", key:21, to:"/staff/customer-service-chat", roles: ["Customer Service"]},
        {text:"Chat Maintenance Account", key:22, to:"/staff/maintenance-account-chat", roles: ["Ride Manager", "CEO"]},
        {text:"Care and Maintenance Official Account", key:23, to:"/staff/maintenance-official-account", roles: ["Maintenance Manager", "CEO"]},
        {text:"View Store", key:24, to:"/staff/view-store", roles: ["Sales Associate"]},
        {text:"View Maintenance", key:25, to:"/staff/view-maintenance", roles: ["Maintenance Staff"]},
        {text:"View Ride Revenue", key:26, to:"/staff/view-ride-revenue", roles: ["Ride Manager", "CFO"]},
    ]


    const showNavItem = (allowedRoles: string[]) => {
        if (!isAuthenticated) return false;
        if (allowedRoles.length === 0) return true;
        return allowedRoles.includes(role as string);
        return true;
    }

    return (
        <main className="fixed bg-purple-500 w-full h-20 top-0 flex flex-row">
            <StaffDashboardMenu/>
            {!isAuthenticated ? (
                <StaffNavButton text="Login" to="/staff/login"/>
            ) : (
                <StaffLogoutMenu/>
            )}
            <StaffNavButton text={"Staff Chat"} to={"/staff/staff-chat"}/>
            {isAuthenticated && (
                <div className="flex flex-row">
                    {navItems.map((item) => (
                        showNavItem(item.roles) && (
                            <StaffNavButton key={item.key} text={item.text} to={item.to}/>
                        )
                    ))}
                </div>
             )}
        </main>
    )
}
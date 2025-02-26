import {StaffNavButton} from "@/apps/staff/components/navbar/StaffNavButton.tsx";
import {useStaffAuth} from "@/context/StaffAuthProvider.tsx";
import {StaffLogoutMenu} from "@/apps/staff/components/navbar/StaffLogoutMenu.tsx";
import {StaffDashboardMenu} from "@/apps/staff/components/navbar/StaffDashboardMenu.tsx";

export default function StaffNavbar(){
    const {role, isAuthenticated} = useStaffAuth();

    const navItems = [
        {text:"Create Staff Account", key:1, to:"/staff/create-account", roles: ["COO"]},
        {text:"Add New Menu", key:2, to:"/staff/add-new-menu", roles: ["F&B Supervisor"]},
        {text:"View All Restaurant", key:3, to:"/staff/view-all-restaurant", roles: ["F&B Supervisor"]},
    ]

    const showNavItem = (allowedRoles: string[]) => {
        // if (!isAuthenticated) return false;
        // if (allowedRoles.length === 0) return true;
        // return allowedRoles.includes(role as string);
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

            {/*Uncommented isAuthenticated, and uncomment the showNavItem function*/}

            {/*{isAuthenticated && (*/}
                <div className="flex flex-row">
                    {navItems.map((item) => (
                        showNavItem(item.roles) && (
                            <StaffNavButton key={item.key} text={item.text} to={item.to}/>
                        )
                    ))}
                </div>
             {/*)}*/}
        </main>
    )
}
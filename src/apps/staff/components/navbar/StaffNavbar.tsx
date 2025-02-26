import {StaffNavButton} from "@/apps/staff/components/navbar/StaffNavButton.tsx";
import {useStaffAuth} from "@/context/StaffAuthProvider.tsx";
import {StaffLogoutMenu} from "@/apps/staff/components/navbar/StaffLogoutMenu.tsx";
import {StaffDashboardMenu} from "@/apps/staff/components/navbar/StaffDashboardMenu.tsx";

export default function StaffNavbar(){
    const {role, isAuthenticated} = useStaffAuth();

    const navItems = [
        {text:"Create Staff Account", key:1, to:"/staff/create-account", roles: ["COO"]},
    ]

    const showNavItem = (allowedRoles: string[]) => {
        if (!isAuthenticated) return false;
        if (allowedRoles.length === 0) return true;
        return allowedRoles.includes(role as string);
    }

    return (
        <main className="fixed bg-purple-500 w-full h-20 top-0 flex flex-row">
            <StaffDashboardMenu/>
            {!isAuthenticated ? (
                <StaffNavButton text="Login" to="/staff/login"/>
            ) : (
                <StaffLogoutMenu/>
            )}

            {/*Remove || true to enable middleware*/}

            {isAuthenticated || true && (
                <div className="flex flex-row">
                    {navItems.map((item) => (
                        showNavItem(item.roles) || true && (
                            <StaffNavButton key={item.key} text={item.text} to={item.to}/>
                        )
                    ))}
                </div>
            )}
        </main>
    )
}
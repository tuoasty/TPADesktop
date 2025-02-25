import {StaffNavButton} from "@/apps/staff/components/navbar/StaffNavButton.tsx";
import {useStaffAuth} from "@/context/StaffAuthProvider.tsx";
import {StaffLogout} from "@/apps/staff/components/navbar/StaffLogout.tsx";

export default function StaffNavbar(){
    const {role, isAuthenticated} = useStaffAuth();

    const navItems = [
        {text:"Create Staff Account", to:"/create-staff-account", roles: ["COO"]},
    ]

    const showNavItem = (allowedRoles: string[]) => {
        if (!isAuthenticated) return false;
        if (allowedRoles.length === 0) return true;
        return allowedRoles.includes(role as string);
    }

    return (
        <main className="fixed bg-blue-500 w-full h-20 top-0 flex flex-row">
            <div className="pl-8 pr-8 justify-center items-center flex">
                <h1 className="text-white text-center place-items-center font-bold text-xl">{isAuthenticated ? role : "Hello"}</h1>
            </div>
            {!isAuthenticated ? (
                <StaffNavButton text="Login" to="/login"/>
            ) : (
                <StaffLogout/>
            )}
            {isAuthenticated && (
                <div className="flex flex-row">
                    {navItems.map((item) => (
                        showNavItem(item.roles) && (
                            <StaffNavButton text={item.text} to={item.to}/>
                        )
                    ))}
                </div>
            )}
        </main>
    )
}
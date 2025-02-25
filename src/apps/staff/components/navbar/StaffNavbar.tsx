import {StaffNavButton} from "@/apps/staff/components/navbar/StaffNavButton.tsx";

export default function StaffNavbar(){
    return (
        <main className="fixed bg-blue-500 w-full h-20 top-0 flex flex-row">
            <StaffNavButton text="Ini Logo" to="/dashboard"/>
            <StaffNavButton text="Login" to="/login"/>
            <StaffNavButton text="Register" to="/register"/>
        </main>
    )
}
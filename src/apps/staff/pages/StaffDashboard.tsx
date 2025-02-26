import {useStaffAuth} from "@/context/StaffAuthProvider.tsx";

export default function StaffDashboard(){

    const { username, role } = useStaffAuth();

    return (
        <main className="bg-purple-700 h-full w-full flex flex-col place-items-center justify-center">
            <h1 className="text-white font-bold text-3xl">Welcome {username}!</h1>
            <h1 className="text-white font-bold text-3xl">{role}</h1>
        </main>
    )
}
import {useAuth} from "@/auth/AuthProvider.tsx";

export default function StaffDashboard(){

    const { username } = useAuth();

    return (
        <main>
            <h1>Name : {username}</h1>
        </main>
    )
}
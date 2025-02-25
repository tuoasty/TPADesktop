import {useAuth} from "@/auth/StaffAuthProvider.tsx";

export default function StaffDashboard(){

    const { username, role } = useAuth();

    return (
        <main>
            <h1>Name : {username}</h1>
            <h1>Role : {role}</h1>
        </main>
    )
}
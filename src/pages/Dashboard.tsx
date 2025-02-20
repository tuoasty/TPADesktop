import {useAuth} from "@/auth/AuthProvider.tsx";

export default function Dashboard(){

    const { username } = useAuth();

    return (
        <main>
            <h1>Name : {username}</h1>
        </main>
    )
}
import {useCustomerAuth} from "@/context/CustomerAuthProvider.tsx";

export default function CustomerDashboard(){
    const {name} = useCustomerAuth()

    return (
        <main className="bg-blue-700 h-full w-full flex flex-col place-items-center justify-center">
            <h1 className="text-white font-bold text-3xl">Welcome {name}!</h1>
        </main>
    )
}
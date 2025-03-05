import {useCustomerAuth} from "@/context/CustomerAuthProvider.tsx";

export default function CustomerDashboard(){
    const {name, balance} = useCustomerAuth()

    return (
        <main className="bg-blue-700 h-full w-full flex flex-col place-items-center justify-center">
            <h1 className="text-white font-bold text-3xl">Welcome {name}!</h1>
            <h1 className="text-white font-bold text-3xl">Balance : {balance}</h1>
        </main>
    )
}
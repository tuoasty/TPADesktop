import {useCustomerAuth} from "@/context/CustomerAuthProvider.tsx";

export const CustomerLogout = () => {
    const {customerLogout} = useCustomerAuth();

    return (
        <div onClick={customerLogout} className="pl-6 pr-6 justify-center items-center flex">
            <h1 className="text-white text-center place-items-center font-bold">Logout</h1>
        </div>
    )
}
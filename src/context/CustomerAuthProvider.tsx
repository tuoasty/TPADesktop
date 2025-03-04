import React, {createContext, useContext, useEffect, useState} from "react";
import {toast} from "sonner";
import {invoke} from "@tauri-apps/api/core";
import {useNavigate} from "react-router-dom";

const CustomerAuthContext = createContext<CustomerAuthContextType | null>(null)

type CustomerAuthContextType = {
    customerId:number | null;
    name:string | null;
    balance:number | null;
    customerIsLoggedIn:() => Promise<boolean>;
    customerLogout:() => Promise<void>;
    getCurrentCustomer:() => Promise<void>;
}
export const CustomerAuthProvider = ({children} : {children:React.ReactNode}) => {
    const [customerId, setCustomerId] = useState<number | null>(null);
    const [name, setName] = useState<string | null>(null);
    const [balance, setBalance] = useState<number | null>(null);

    const customerIsLoggedIn = async(): Promise<boolean> => {
        try {
            return await invoke<boolean>("verify_customer_login");
        } catch (e) {
            toast.error(`${e}`)
            return false;
        }
    }

    const customerLogout = async() => {
        try {
            await invoke("logout_customer");
            setCustomerId(null);
            setName(null);
            setBalance(null);
        } catch (e) {
            toast.error(`${e}`)
        }
    }

    const getCurrentCustomer = async () => {
        try {
            const result = await invoke<[number, string, number]>("get_current_customer");
            if(result){
                setCustomerId(result[0])
                setName(result[1])
                setBalance(result[2])
            }
        } catch {
            setCustomerId(null);
            setName(null);
            setBalance(null);
        }
    }

    const value:CustomerAuthContextType = {
        customerId,
        name,
        balance,
        customerIsLoggedIn,
        customerLogout,
        getCurrentCustomer
    }

    return (
        <CustomerAuthContext.Provider value={value}>
            {children}
        </CustomerAuthContext.Provider>
    )
}

export const useCustomerAuth = () => {
    const context = useContext(CustomerAuthContext);
    if(!context){
        throw new Error("error using context");
    }

    return context;
}

export const ProtectedRoute = ({children}:{children:React.ReactNode}) => {
    const {customerIsLoggedIn} = useCustomerAuth();
    let navigate = useNavigate();

    useEffect(() => {
        const checkAuth = async () => {
            try {
                const loggedIn = await customerIsLoggedIn();
                if(!loggedIn){
                    navigate("/login", {replace:true});
                    return;
                }
            } catch (e) {
                navigate("/login", {replace:true});
            }
        }
        checkAuth();
    }, [customerIsLoggedIn, navigate]);

    return <>{children}</>
}
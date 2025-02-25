import {createContext, useContext, useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {Navigate, useNavigate} from "react-router-dom";
import NotAuthorized from "@/NotAuthorized.tsx";

const StaffAuthContext = createContext<StaffAuthContextType | null>(null)

type StaffAuthContextType = {
    staffId:number | null;
    username: string | null;
    role: string | null;
    isAuthenticated: boolean;
    logout: () => Promise<void>;
    checkAuth: () => Promise<void>;
}
export const StaffAuthProvider = ({children} : {children:React.ReactNode}) => {
    const [staffId, setStaffId] = useState<number | null>(null);
    const [username, setUsername] = useState<string | null>(null);
    const [role, setRole] = useState<string | null>(null);
    const navigate = useNavigate();

    useEffect(() => {
        checkAuth();
    }, []);

    const checkAuth = async () => {
        try {
            const result = await invoke<[number, string, string]>("get_staff");
            if(result){
                setStaffId(result[0])
                setUsername(result[1])
                setRole(result[2])
            }
        } catch {
            setUsername(null)
            setStaffId(null)
            setRole(null)
        }
    }

    const logout = async () => {
        try {
            await invoke("logout_staff");
            setStaffId(null);
            setUsername(null);
            navigate("/login")
        } catch {
            console.error("logout failed");
        }
    }

    const value:StaffAuthContextType = {
        staffId,
        username,
        role,
        isAuthenticated: staffId != null,
        logout,
        checkAuth
    };

    return (
        <StaffAuthContext.Provider value={value}>
            {children}
        </StaffAuthContext.Provider>
    )
}

export const useStaffAuth = () => {
    const context = useContext(StaffAuthContext);
    if(!context){
        throw new Error("error using context");
    }
    return context;
}

export const ProtectedRoute = ({children, allowedRoles = []}:{children:React.ReactNode, allowedRoles?:string[]}) => {
    const {isAuthenticated, role} = useStaffAuth();

    if(!isAuthenticated){
        return <Navigate to="/login" replace/>
    }

    if(allowedRoles && allowedRoles.length > 0){
        const hasRole = allowedRoles.includes(role as string);

        if(!hasRole){
            return <NotAuthorized/>
        }
    }

    return <>{children}</>
}


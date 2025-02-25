import {createContext, useContext, useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {Navigate, useNavigate} from "react-router-dom";

const AuthContext = createContext<AuthContextType | null>(null)

type AuthContextType = {
    staffId:number | null;
    username: string | null;
    isAuthenticated: boolean;
    logout: () => Promise<void>;
    checkAuth: () => Promise<void>;
}
export const AuthProvider = ({children} : {children:React.ReactNode}) => {
    const [staffId, setStaffId] = useState<number | null>(null);
    const [username, setUsername] = useState<string | null>(null);
    const navigate = useNavigate();

    useEffect(() => {
        checkAuth();
    }, []);

    const checkAuth = async () => {
        try {
            const result = await invoke<[number, string]>("get_staff");
            if(result){
                setStaffId(result[0])
                setUsername(result[1])
            }
        } catch {
            setUsername(null)
            setStaffId(null)
        }
    }

    const logout = async () => {
        try {
            await invoke("logout");
            setStaffId(null);
            setUsername(null);
            navigate("/login")
        } catch {
            console.error("logout failed");
        }
    }

    const value:AuthContextType = {
        staffId,
        username,
        isAuthenticated: staffId != null,
        logout,
        checkAuth
    };

    return (
        <AuthContext.Provider value={value}>
            {children}
        </AuthContext.Provider>
    )
}

export const useAuth = () => {
    const context = useContext(AuthContext);
    if(!context){
        throw new Error("error using context");
    }
    return context;
}

export const ProtectedRoute = ({children}:{children:React.ReactNode}) => {
    const {isAuthenticated} = useAuth();

    if(!isAuthenticated){
        return <Navigate to="/login" replace/>
    }

    return <>{children}</>
}


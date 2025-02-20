import {createContext, useContext, useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {Navigate, useNavigate} from "react-router-dom";

const AuthContext = createContext<AuthContextType | null>(null)

type AuthContextType = {
    userId:number | null;
    username: string | null;
    isAuthenticated: boolean;
    logout: () => Promise<void>;
    checkAuth: () => Promise<void>;
}
export const AuthProvider = ({children} : {children:React.ReactNode}) => {
    const [userId, setUserId] = useState<number | null>(null);
    const [username, setUsername] = useState<string | null>(null);
    const navigate = useNavigate();

    useEffect(() => {
        checkAuth();
    }, []);

    const checkAuth = async () => {
        try {
            const result = await invoke<[number, string]>("get_user");
            if(result){
                setUserId(result[0])
                setUsername(result[1])
            }
        } catch {
            setUsername(null)
            setUserId(null)
        }
    }

    const logout = async () => {
        try {
            await invoke("logout");
            setUserId(null);
            setUsername(null);
            navigate("/login")
        } catch {
            console.error("logout failed");
        }
    }

    const value:AuthContextType = {
        userId,
        username,
        isAuthenticated: userId != null,
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


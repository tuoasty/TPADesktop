import {createContext, useContext, useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {useNavigate} from "react-router-dom";
import NotAuthorized from "@/NotAuthorized.tsx";

const StaffAuthContext = createContext<StaffAuthContextType | null>(null)

type StaffAuthContextType = {
    staffId:number | null;
    username: string | null;
    role: string | null;
    isAuthenticated:boolean;
    hasPermission : (allowedRoles:string[]) => Promise<boolean>;
    isLoggedIn: () => Promise<boolean>;
    logoutStaff: () => Promise<void>;
    getCurrentStaff: () => Promise<void>;
}
export const StaffAuthProvider = ({children} : {children:React.ReactNode}) => {
    const [staffId, setStaffId] = useState<number | null>(null);
    const [username, setUsername] = useState<string | null>(null);
    const [role, setRole] = useState<string | null>(null);
    const navigate = useNavigate();

    useEffect(() => {
        getCurrentStaff();
    }, []);

    const hasPermission = async (allowedRoles:string[] = []):Promise<boolean> => {
        try {
            return await invoke<boolean>("verify_authentication", {allowedRoles});
        } catch (e) {
            console.error(e);
            return false;
        }
    }

    const isLoggedIn = async ():Promise<boolean> => {
        try {
            return await invoke<boolean>("verify_login");
        } catch (e) {
            console.error(e);
            return false;
        }
    }

    const getCurrentStaff = async () => {
        try {
            const result = await invoke<[number, string, string]>("get_current_staff");
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

    const logoutStaff = async () => {
        try {
            await invoke("logout_staff");
            setStaffId(null);
            setUsername(null);
            setRole(null);
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
        hasPermission,
        isLoggedIn,
        logoutStaff,
        getCurrentStaff
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
    const {hasPermission, isLoggedIn} = useStaffAuth();
    let navigate = useNavigate();
    const [authorized, setAuthorized] = useState(false);
    const [loading, setLoading] = useState(true);


    useEffect(() => {
        const checkAuth = async () => {
            try {
                const loggedIn = await isLoggedIn();
                if(!loggedIn){
                    navigate("/login", {replace:true});
                    return;
                }

                const isAuth = await hasPermission(allowedRoles);
                setAuthorized(isAuth);
            } catch (e) {
                navigate("/login", {replace:true});
            } finally {
                setLoading(false);
            }
        }

        checkAuth();
    }, [isLoggedIn, hasPermission, allowedRoles, navigate]);
    if(loading){
        return <h1>Loading...</h1>
    }

    if(!authorized){
        return <NotAuthorized/>
    }
    return <>{children}</>
}


import {useStaffAuth} from "@/context/StaffAuthProvider.tsx";

export const StaffLogoutMenu= () => {
    const {logoutStaff} = useStaffAuth()

    return (
        <button onClick={logoutStaff} className="pl-8 pr-8 justify-center items-center flex">
            <h1 className="text-white text-center place-items-center font-bold text-xl">Logout</h1>
        </button>
    )
}
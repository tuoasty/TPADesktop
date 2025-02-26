import {Link} from "react-router-dom";


export const StaffDashboardMenu= () => {
    return (
        <Link to="/staff/" className="pl-8 pr-8 justify-center items-center flex">
            <h1 className="text-white text-center place-items-center font-bold text-xl">Home</h1>
        </Link>
    )
}
import {FC} from "react";
import {Link, LinkProps} from "react-router-dom";

interface Props {
    text:String;
    to: LinkProps['to']
}
export const StaffNavButton:FC<Props> = (p) => {
    return (
        <Link to={p.to} className="pl-8 pr-8 justify-center items-center flex">
            <h1 className="text-white text-center place-items-center font-bold text-xl">{p.text}</h1>
        </Link>
    )
}
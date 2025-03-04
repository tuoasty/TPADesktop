import {FC} from "react";
import {Link, LinkProps} from "react-router-dom";

interface Props {
    text: String;
    to: LinkProps['to']
}

export const CustomerNavButton: FC<Props> = (p) => {
    return (
        <Link to={p.to} className="pl-6 pr-6 justify-center items-center flex">
            <h1 className="text-white text-center place-items-center font-bold">{p.text}</h1>
        </Link>
    )
}
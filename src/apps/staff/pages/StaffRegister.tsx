import {Input} from "@/components/ui/input.tsx";
import {Button} from "@/components/ui/button.tsx";
import {useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {Link} from "react-router-dom";

export default function StaffRegister(){
    const [formData, setFormData] = useState({
        username:"",
        password:"",
        role:"",
    })

    const [responseMessage, setResponseMessage] = useState("");
    function handleInputChange(e: React.ChangeEvent<HTMLInputElement>){
        setFormData({...formData, [e.target.name]: e.target.value});
    }

    async function handleRegister(e: React.FormEvent){
        e.preventDefault();

        const result : string = await invoke("register_staff", {
            username:formData.username,
            inputPassword:formData.password,
            role:formData.role
        })

        setResponseMessage(result);
    }

    return (
        <main className="h-full w-full flex justify-center items-center">
            <form className="bg-gray-400 w-[30%] h-[75%] flex flex-col justify-center items-center p-5 gap-5"
            onSubmit={handleRegister}>
                <label className="text-3xl font-bold">Register</label>
                <Input type="text" placeholder="Username" name="username" value={formData.username}
                       onChange={handleInputChange}/>
                <Input type="password" placeholder="Password" name="password" value={formData.password}
                       onChange={handleInputChange}/>
                <Input type="text" placeholder="Role" name="role" value={formData.role}
                       onChange={handleInputChange}/>
                <Link to="/login">
                    <label>Have an account? Login</label>
                </Link>
                <Button className="w-full">Register</Button>
            </form>
            <label>{responseMessage}</label>
        </main>
    )
}
import {Button} from "@/components/ui/button.tsx";
import {Input} from "@/components/ui/input.tsx";
import {useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {useNavigate} from "react-router-dom";
import {useStaffAuth} from "@/context/StaffAuthProvider.tsx";

export default function StaffLogin() {
    const {checkAuth} = useStaffAuth();
    const [formData, setFormData] = useState({
        username:"",
        password:"",
    })
    const [message, setMessage] = useState("")
    const navigate = useNavigate()

    function handleInputChange(e: React.ChangeEvent<HTMLInputElement>){
        setFormData({...formData, [e.target.name]: e.target.value});
    }

    async function handleLogin(event: React.FormEvent){
        event.preventDefault()
        try {
            const result:string = await invoke("login_staff", {
                username:formData.username,
                inputPassword:formData.password
            })
            await checkAuth();
            setMessage(result)
            navigate("/dashboard")
        } catch {
            setMessage("Incorrect credentials")
        }
    }

    return (
        <main className={"w-full h-full flex flex-col items-center justify-center bg-blue-300"}>
            <form className={"w-96 h-96 flex flex-col items-center justify-center bg-white gap-3 p-10 rounded-2xl"}
            onSubmit={handleLogin}>
                <h1 className={"text-black font-bold text-3xl"}>Staff Login</h1>
                <Input type={"text"} placeholder={"Username"} name="username" value={formData.username}
                onChange={handleInputChange}/>
                <Input type={"password"} placeholder={"Password"} name="password" value={formData.password}
                onChange={handleInputChange}/>
                <Button className={"w-full bg-blue-500 text-white"}>Login</Button>
                <label className="text-red-700">{message}</label>
            </form>
        </main>
    )
}
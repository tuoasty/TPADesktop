import {Button} from "@/components/ui/button.tsx";
import {Checkbox} from "@/components/ui/checkbox.tsx";
import {Input} from "@/components/ui/input.tsx";
import {useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {Link, useNavigate} from "react-router-dom";
import {useAuth} from "@/auth/AuthProvider.tsx";

export default function LoginPage() {
    const {checkAuth} = useAuth();
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
            const result:string = await invoke("login_user", {
                username:formData.username,
                inputPassword:formData.password
            })
            checkAuth();
            setMessage(result)
            navigate("/dashboard")
        } catch {
            setMessage("Incorrect credentials")
        }
    }

    return (
        <main className={"w-full h-screen flex flex-col items-center justify-center"}>
            <form className={"w-96 h-96 flex flex-col items-center justify-center bg-cyan-800 gap-3 p-10"}
            onSubmit={handleLogin}>
                <h1 className={"text-white"}>Login Page</h1>
                <Input type={"text"} placeholder={"Username"} name="username" value={formData.username}
                onChange={handleInputChange}/>
                <Input type={"password"} placeholder={"Password"} name="password" value={formData.password}
                onChange={handleInputChange}/>
                <div className={"flex items-center gap-2"}>
                    <Checkbox/>
                    <label className={"text-white"}>Accept the terms and conditions</label>
                </div>
                <Button className={"w-72 bg-white text-cyan-800"}>Login</Button>
                <label className="text-white">{message}</label>
                <Link to="/register">
                    <label>Don't have an account? Register here</label>
                </Link>
            </form>
        </main>
    )
}
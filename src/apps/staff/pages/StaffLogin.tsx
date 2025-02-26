import {Button} from "@/components/ui/button.tsx";
import {Input} from "@/components/ui/input.tsx";
import {useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {useNavigate} from "react-router-dom";
import {useStaffAuth} from "@/context/StaffAuthProvider.tsx";
import {toast} from "sonner";

export default function StaffLogin() {
    const {getCurrentStaff} = useStaffAuth();
    const [formData, setFormData] = useState({
        name:"",
        password:"",
    })
    const navigate = useNavigate()

    function handleInputChange(e: React.ChangeEvent<HTMLInputElement>){
        setFormData({...formData, [e.target.name]: e.target.value});
    }

    async function loginStaff(event: React.FormEvent){
        event.preventDefault()
        try {
            await invoke("login_staff", {
                name:formData.name,
                password:formData.password
            })
            await getCurrentStaff();
            toast.success("Successfull login!")
            navigate("/staff/")
        } catch {
            toast.error("Incorrect Credentials")
        }
    }

    return (
        <main className={"w-full h-full flex flex-col items-center justify-center bg-purple-200"}>
            <form className={"w-96 h-96 flex flex-col items-center justify-center bg-white gap-3 p-10 rounded-2xl"}
            onSubmit={loginStaff}>
                <h1 className={"text-black font-bold text-3xl"}>Staff Login</h1>
                <Input type={"text"} placeholder={"Username"} name="name" value={formData.name}
                onChange={handleInputChange}/>
                <Input type={"password"} placeholder={"Password"} name="password" value={formData.password}
                onChange={handleInputChange}/>
                <Button className={"w-full bg-purple-500 text-white"}>Login</Button>
            </form>
        </main>
    )
}
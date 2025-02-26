import {Input} from "@/components/ui/input.tsx";
import {Button} from "@/components/ui/button.tsx";
import {useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {toast} from "sonner";

export default function CreateStaffAccount(){
    const [formData, setFormData] = useState({
        username:"",
        password:"",
        role:"",
    })
    function handleInputChange(e: React.ChangeEvent<HTMLInputElement>){
        setFormData({...formData, [e.target.name]: e.target.value});
    }

    async function handleRegister(e: React.FormEvent){
        e.preventDefault();

        await invoke("register_staff", {
            username:formData.username,
            inputPassword:formData.password,
            userRole:formData.role
        })

        toast("Successfully created account");
    }

    return (
        <main className="h-full w-full bg-purple-200 flex justify-center items-center">
            <form className="bg-white w-[30%] h-[75%] flex flex-col justify-center items-center p-5 gap-5 rounded-2xl"
            onSubmit={handleRegister}>
                <label className="text-3xl font-bold">Create Staff Account</label>
                <Input type="text" placeholder="Username" name="username" value={formData.username}
                       onChange={handleInputChange}/>
                <Input type="password" placeholder="Password" name="password" value={formData.password}
                       onChange={handleInputChange}/>
                <Input type="text" placeholder="Role" name="role" value={formData.role}
                       onChange={handleInputChange}/>
                <Button className="w-full bg-purple-500 text-white bold ">Register</Button>
            </form>
        </main>
    )
}
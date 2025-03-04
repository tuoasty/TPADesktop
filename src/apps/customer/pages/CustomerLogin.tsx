import {Input} from "@/components/ui/input.tsx";
import {Button} from "@/components/ui/button.tsx";
import {invoke} from "@tauri-apps/api/core";
import {toast} from "sonner";
import {useState} from "react";
import {useNavigate} from "react-router-dom";
import {useCustomerAuth} from "@/context/CustomerAuthProvider.tsx";
export default function CustomerLogin(){
    const {getCurrentCustomer} = useCustomerAuth();
    const [formData, setFormData] = useState({
        id:"",
        name:"",
    })
    const navigate = useNavigate()

    function handleInputChange(e: React.ChangeEvent<HTMLInputElement>){
        setFormData({...formData, [e.target.name]: e.target.value});
    }

    const loginCustomer = async (event:React.FormEvent) => {
        event.preventDefault()
        try {
            await invoke("login_customer", {id:parseInt(formData.id, 0), name:formData.name})
            await getCurrentCustomer();
            toast.success("Success");
            navigate("/customer/")
        } catch (e) {
            console.log(e)
            toast.error(`${e}`)
        }
    }

    return (
        <main className={"w-full h-full flex flex-col items-center justify-center bg-blue-300"}>
            <form className={"w-96 h-96 flex flex-col items-center justify-center bg-white gap-3 p-10 rounded-2xl"}
            onSubmit={loginCustomer}>
                <h1 className={"text-black font-bold text-3xl"}>Customer Login</h1>
                <Input type={"text"} placeholder={"ID"} name="id" value={formData.id}
                       onChange={handleInputChange}/>
                <Input type={"text"} placeholder={"Name"} name="name" value={formData.name}
                       onChange={handleInputChange}/>
                <Button className={"w-full bg-blue-500 text-white"}>Login</Button>
            </form>
        </main>
    )
}
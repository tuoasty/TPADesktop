import {Input} from "@/components/ui/input.tsx";
import {Button} from "@/components/ui/button.tsx";
import {useForm} from "react-hook-form";
import {Customer} from "@/ types/customer.ts";

export default function CustomerLogin(){
    const {register, handleSubmit, reset} = useForm<Customer>();

    return (
        <main className={"w-full h-full flex flex-col items-center justify-center bg-blue-300"}>
            <form className={"w-96 h-96 flex flex-col items-center justify-center bg-white gap-3 p-10 rounded-2xl"}>
                <h1 className={"text-black font-bold text-3xl"}>Customer Login</h1>
                <Input id="id" {...register("id")} placeholder="Customer ID"/>
                <Input id="name" {...register("name")} placeholder="Name"/>
                <Button className={"w-full bg-blue-500 text-white"}>Login</Button>
            </form>
        </main>
    )
}
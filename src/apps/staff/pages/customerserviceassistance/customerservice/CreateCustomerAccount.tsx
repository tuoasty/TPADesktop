import { useForm } from "react-hook-form";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { invoke } from "@tauri-apps/api/core";
import { toast } from "sonner";

export default function CreateCustomerAccount() {
    const { register, handleSubmit } = useForm();

    async function createCustomer(data: any) {
        try {
            let id = invoke("create_customer_account", {data:data})
            toast.success(`Created customer account with ID: ${id}`)
        } catch (e) {
            toast.error(`${e}`)
        }
    }

    return (
        <main className="h-full w-full bg-purple-200 flex justify-center items-center">
            <form
                className="bg-white w-[25%] h-auto flex flex-col justify-center items-center p-12 gap-5 rounded-2xl"
                onSubmit={handleSubmit(createCustomer)}
            >
                <Input
                    type="text"
                    placeholder="Name"
                    {...register("name", { required: true })}
                />
                <Input
                    type="number"
                    placeholder="Balance"
                    {...register("balance", { required: true })}
                />
                <Button type="submit">Create Account</Button>
            </form>
        </main>
    );
}

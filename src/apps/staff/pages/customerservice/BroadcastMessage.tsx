import {Input} from "@/components/ui/input.tsx";
import {Button} from "@/components/ui/button.tsx";
import {useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {toast} from "sonner";

export default function BroadcastMessage(){
    const [message, setMessage] = useState("");
    const broadcastMessage = async () => {
        try {
            await invoke("broadcast_message", {newMessage:message})
            toast.success("Successfully created broadcast message")
        } catch (e) {
            toast.error(`${e}`)
        }
    }

    return (
        <div className="h-full w-full bg-purple-200 flex justify-center place-items-center">
            <form
                className="bg-white h-auto w-[25%] flex flex-col justify-center place-items-center gap-5 p-12 rounded-2xl"
                onSubmit={broadcastMessage}>
                <h1 className="font-bold text-2xl">Broadcast Message</h1>
                <Input type="text" placeholder="Message" name="message" onChange={(val) => setMessage(val.target.value)}/>
                <Button className="bg-purple-500 w-full">Add Menu</Button>
            </form>
        </div>
    )
}
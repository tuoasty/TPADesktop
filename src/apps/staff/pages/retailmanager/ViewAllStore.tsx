import {useEffect, useState} from "react";
import {Store} from "@/ types/store.ts";
import {invoke} from "@tauri-apps/api/core";
import {Souvenir} from "@/ types/souvenir.ts";
import {
    AlertDialog, AlertDialogAction, AlertDialogCancel,
    AlertDialogContent, AlertDialogDescription, AlertDialogFooter,
    AlertDialogHeader,
    AlertDialogTitle,
    AlertDialogTrigger
} from "@/components/ui/alert-dialog.tsx";
import {toast} from "sonner";
import {Button} from "@/components/ui/button.tsx";
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger
} from "@/components/ui/dialog.tsx";
import {Label} from "@/components/ui/label.tsx";
import {Input} from "@/components/ui/input.tsx";

export default function ViewAllStore() {
    const [stores, setStores] = useState<Store[]>([]);

    const fetchStores = async () => {
        invoke<Store[]>("find_all_store")
            .then(setStores)
    }

    useEffect(() => {
        fetchStores();
    }, []);

    const removeSouvenir = async (id: number) => {
        try {
            await invoke("remove_souvenir", {selectedId: id});
            toast.success("Successfully removed")
            await fetchStores();
        } catch (error) {
            toast.error(`${error}`)
        }
    }

    const updateStatus = (id:number, status:string) => {
        try {
            invoke("change_store_status", {storeId:id, storeStatus:status}).then(() => {
                toast.success("Successfully updated store status");
                fetchStores();
            })

        } catch (e) {
            toast.error(`${e}`)
        }
    }

    return (
        <div className="h-screen w-full bg-purple-200 flex flex-col p-16 overflow-auto gap-5">
            {stores.length > 0 && (
                stores.map((store: Store) => (
                    <div key={store.id} className="w-full bg-white h-min-72 rounded-2xl shrink-0 flex">
                        <div className="w-96 h-auto p-8 overflow-hidden">
                            <img className="object-contain w-full h-full rounded-lg" src={store.image_data}
                                 alt={store.name}/>
                        </div>
                        <div className="w-full h-full flex flex-col p-8 gap-2">
                            <div className="w-full h-auto flex flex-row justify-between mb-5">
                                <div>
                                    <h1 className="font-bold text-4xl">{store.name}</h1>
                                    <h4>Open Time : {store.open_time} - {store.close_time}</h4>
                                    <h3 className="font-bold text-2xl">Souvenirs</h3>
                                    <h3>Status : {store.status}</h3>
                                </div>
                                <div className="w-48 h-full gap-4 flex flex-col">
                                    <Dialog>
                                        <DialogTrigger asChild>
                                            <Button className="bg-purple-700 w-48 h-12">Assign Staff</Button>
                                        </DialogTrigger>
                                        <DialogContent>
                                            <DialogHeader>
                                                <DialogTitle>Maintenance Request</DialogTitle>
                                                <DialogDescription>Enter maintenance description</DialogDescription>
                                            </DialogHeader>
                                            <div className="grid gap-4 py-4">
                                                <div className="grid grid-cols-4 items-center gap-4">
                                                    <Label htmlFor="reason" className="text-right">
                                                        Reasoning
                                                    </Label>
                                                    <Input id="reason" type="text" className="col-span-3" />
                                                </div>
                                            </div>
                                            <DialogFooter>
                                                <Button type="submit" className="bg-purple-700">Save changes</Button>
                                            </DialogFooter>
                                        </DialogContent>
                                    </Dialog>
                                    <Button
                                        onClick={() => updateStatus(store.id, store.status)}
                                        className={`w-48 h-12 ${store.status == "Closed" ? "bg-green-500" : "bg-red-500"}`}>
                                        {store.status == "Closed" ? "Open" : "Close"}
                                    </Button>
                                </div>
                            </div>
                            {store.souvenirs.length > 0 && (
                                store.souvenirs.map((souvenir: Souvenir) => (
                                    <div key={souvenir.id} className="bg-purple-200 rounded-2xl p-2 flex justify-between">
                                        <div className="flex flex-row">
                                            <div className="h-28 w-28 mr-4 overflow-hidden">
                                                <img className="object-cover w-full h-full rounded-lg"
                                                     src={souvenir.image_data}
                                                     alt={souvenir.name}/>
                                            </div>
                                            <div className="flex justify-center flex-col">
                                                <h4 className="font-bold">{souvenir.name}</h4>
                                                <h4>Price : {souvenir.price}</h4>
                                                <h4>{souvenir.description}</h4>
                                            </div>
                                        </div>
                                        <div className="flex justify-center place-items-center mr-4">
                                            <AlertDialog>
                                                <AlertDialogTrigger asChild>
                                                    <Button type="submit" className="bg-red-500">Remove Souvenir</Button>
                                                </AlertDialogTrigger>
                                                <AlertDialogContent>
                                                    <AlertDialogHeader>
                                                        <AlertDialogTitle>Delete Souvenir?</AlertDialogTitle>
                                                        <AlertDialogDescription>
                                                            This action cannot be undone.
                                                        </AlertDialogDescription>
                                                    </AlertDialogHeader>
                                                    <AlertDialogFooter>
                                                        <AlertDialogCancel>Cancel</AlertDialogCancel>
                                                        <AlertDialogAction
                                                            onClick={() => removeSouvenir(souvenir.id)}>Confirm</AlertDialogAction>
                                                    </AlertDialogFooter>
                                                </AlertDialogContent>
                                            </AlertDialog>
                                        </div>
                                    </div>
                                ))
                            )}
                        </div>
                    </div>

                ))
            )}
        </div>
    )
}
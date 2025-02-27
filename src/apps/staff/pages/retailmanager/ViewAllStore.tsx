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

export default function ViewAllStore() {
    const [stores, setStores] = useState<Store[]>([]);

    const refreshPage = () => {
        window.location.reload();
    };

    useEffect(() => {
        invoke<Store[]>("find_all_store")
            .then(setStores)
    }, []);

    const removeSouvenir = async (id: number) => {
        try {
            await invoke("remove_souvenir", {souvenirId:id});
            toast.success("Successfully removed")
            refreshPage()
        } catch (error) {
            toast.error(`${error}`)
        }
    }

    return (
        <div className="h-screen w-full bg-purple-200 flex flex-col p-16 overflow-auto gap-5">
            {stores.length > 0 && (
                stores.map((store: Store) => (
                    <div key={store.id} className="w-full bg-white h-min-72 rounded-2xl shrink-0 flex">
                        <div className="w-2xl h-full p-8 overflow-hidden">
                            <img className="object-cover w-full h-full rounded-lg" src={store.image_data}
                                 alt={store.name}/>
                        </div>
                        <div className="w-full h-full flex flex-col p-8 gap-2">
                            <h1 className="font-bold text-4xl">{store.name}</h1>
                            <h4>Open Time : {store.open_time} - {store.close_time}</h4>
                            <h3 className="font-bold text-2xl">Souvenirs</h3>
                            {store.souvenirs.length > 0 && (
                                store.souvenirs.map((souvenir: Souvenir) => (
                                    <div className="bg-purple-200 rounded-2xl p-2 flex justify-between">
                                        <div className="flex justify-center place-items-center flex-col">
                                            <h4 className="font-bold">{souvenir.name}</h4>
                                            <h4>Price : {souvenir.price}</h4>
                                            <h4>{souvenir.description}</h4>
                                        </div>
                                        <div className="flex justify-center place-items-center mr-6">
                                            <AlertDialog>
                                                <AlertDialogTrigger className="bg-red-500 p-2 rounded-xl">
                                                    Remove Souvenir
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
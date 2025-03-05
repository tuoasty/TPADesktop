import {useEffect, useState} from "react";
import {Store} from "@/ types/store.ts";
import {invoke} from "@tauri-apps/api/core";
import {toast} from "sonner";
import {Souvenir} from "@/ types/souvenir.ts";
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger
} from "@/components/ui/dialog.tsx";
import {Button} from "@/components/ui/button.tsx";
import {Label} from "@/components/ui/label.tsx";
import {Input} from "@/components/ui/input.tsx";
import {useCustomerAuth} from "@/context/CustomerAuthProvider.tsx";
import {useNavigate} from "react-router-dom";

interface Props {
    storeId:number;
}
export default function StoreDashboard(p:Props){
    const [store, setStore] = useState<Store | null>(null);
    const [price, setPrice] = useState(0);
    const [count, setCount] = useState(0);
    const navigate = useNavigate();

    const {name, balance, customerIsLoggedIn} = useCustomerAuth();

    const calculatePrice = (val:number, price:number) => {
        if (val) {
            setPrice(val * price)
            setCount(val)
        } else {
            setPrice(0)
            setCount(0)
        }
    }

    const purchaseSouvenir = async (souvenirId:number) => {
        if (!await customerIsLoggedIn()) {
            toast.error("You must be logged in to purchase")
            navigate("login")
            return;
        }

        try {
            await invoke("purchase_souvenir", {souvenirId:souvenirId, souvenirCount:count});
            toast.success("Successfully purchased item");
        } catch (e) {
            toast.error(`${e}`)
        }
    }

    const fetchStore = async () => {
        try {
            invoke<Store>("find_store_by_id", {selectedId:p.storeId}).then(setStore);
        } catch (e) {
            toast.error(`${e}`)
        }
    }

    useEffect(() => {
        fetchStore();
    }, []);

    return (
        <main className="bg-yellow-200 h-full w-full flex flex-col place-items-center justify-center">
            {store && store.status == "Open" ? (
                <div className="h-full w-full flex">
                    <div className="w-[25%] h-auto">
                        <img className="object-contain w-full h-full rounded-lg" src={store.image_data}
                             alt={store.name}/>
                    </div>
                    <div className="w-full p-12 gap-5">
                        {name && (
                            <div>
                                <h1>Welcome {name}</h1>
                                <h1>Balance : {balance}</h1>
                            </div>
                        )}
                        <h1 className="text-2xl font-bold">{store.name}</h1>
                        {store.souvenirs.length > 0 && (
                            store.souvenirs.map((souvenir:Souvenir) => (
                                <div className="w-auto gap-5 m-12">
                                    <div key={souvenir.id}
                                         className="bg-white rounded-2xl p-2 flex justify-between w-full gap-5">
                                        <div className="flex flex-row w-full">
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
                                        <div className="flex flex-col place-items-center justify-center p-8">
                                            <Dialog>
                                                <DialogTrigger asChild>
                                                    <Button disabled={store.status == "Closed" || store.status=="Shut Down"}
                                                        className="bg-yellow-600 w-48 h-12">Purchase</Button>
                                                </DialogTrigger>
                                                <DialogContent>
                                                    <DialogHeader>
                                                        <DialogTitle>Choose Amount</DialogTitle>
                                                        <DialogDescription>Choose Amount to purchase</DialogDescription>
                                                    </DialogHeader>
                                                    <div className="grid gap-4 py-4">
                                                        <div className="grid grid-cols-4 items-center gap-4">
                                                            <Label htmlFor="amount" className="text-right">
                                                               Amount
                                                            </Label>
                                                            <Input id="amount" type="number" className="col-span-3" onChange={(val) => calculatePrice(parseInt(val.target.value), souvenir.price)}/>
                                                        </div>
                                                    </div>
                                                    <DialogFooter className="w-full flex-col justify-between">
                                                        <h2>Price : {price}</h2>
                                                        <Button type="submit" className="bg-yellow-700" onClick={() => purchaseSouvenir(souvenir.id)}
                                                        disabled={store.status == "Closed" || store.status=="Shut Down"}>Purchase</Button>
                                                    </DialogFooter>
                                                </DialogContent>
                                            </Dialog>
                                        </div>
                                    </div>
                                </div>
                            ))
                        )}
                    </div>
                </div>
            ) : (
                <h1>Store is {store?.status}</h1>
            )}
        </main>
    )
}
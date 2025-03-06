import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { toast } from "sonner";
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger
} from "@/components/ui/dialog.tsx";
import { Button } from "@/components/ui/button.tsx";
import { Label } from "@/components/ui/label.tsx";
import { Input } from "@/components/ui/input.tsx";
import { useCustomerAuth } from "@/context/CustomerAuthProvider.tsx";
import { useNavigate } from "react-router-dom";
import {Store} from "@/ types/store.ts";
import {Souvenir} from "@/ types/souvenir.ts";

interface Props {
    storeId: number;
}

export default function StoreDashboard({ storeId }: Props) {
    const [store, setStore] = useState<Store | null>(null);
    const [price, setPrice] = useState(0);
    const [count, setCount] = useState(0);
    const navigate = useNavigate();

    const { name, balance, customerIsLoggedIn } = useCustomerAuth();

    const calculatePrice = (val: number, price: number) => {
        setCount(val || 0);
        setPrice(val ? val * price : 0);
    };

    const purchaseSouvenir = async (souvenirId: number) => {
        if (!(await customerIsLoggedIn())) {
            toast.error("You must be logged in to purchase");
            navigate("login");
            return;
        }

        try {
            await invoke("purchase_souvenir", { souvenirId, souvenirCount: count });
            toast.success("Successfully purchased item");
        } catch (e) {
            toast.error(`${e}`);
        }
    };

    const fetchStore = async () => {
        try {
            invoke<Store>("find_store_by_id", { selectedId: storeId }).then(setStore);
        } catch (e) {
            toast.error(`${e}`);
        }
    };

    useEffect(() => {
        fetchStore();
    }, []);

    return (
        <main className="bg-yellow-100 min-h-screen flex items-center justify-center p-6">
            {store && store.status === "Open" ? (
                <div className="bg-white shadow-lg rounded-xl overflow-hidden w-full max-w-5xl flex flex-col md:flex-row p-6 gap-6">
                    <div className="w-full md:w-1/3">
                        <img className="object-cover w-full h-64 rounded-lg" src={store.image_data} alt={store.name} />
                    </div>

                    <div className="w-full md:w-2/3 flex flex-col justify-between gap-4">
                        {name && (
                            <div className="text-gray-800">
                                <h1 className="text-lg font-semibold">Welcome, {name}!</h1>
                                <p className="text-gray-600">Balance: <span className="font-bold">${balance}</span></p>
                            </div>
                        )}

                        <h1 className="text-3xl font-bold text-gray-900">{store.name}</h1>

                        <div className="space-y-6">
                            {store.souvenirs.length > 0 ? (
                                store.souvenirs.map((souvenir: Souvenir) => (
                                    <div key={souvenir.id} className="bg-gray-50 rounded-xl p-4 flex flex-col md:flex-row gap-6 shadow">
                                        <div className="w-full md:w-32 h-32 overflow-hidden">
                                            <img className="object-cover w-full h-full rounded-lg" src={souvenir.image_data} alt={souvenir.name} />
                                        </div>

                                        <div className="flex-1">
                                            <h4 className="font-bold text-lg">{souvenir.name}</h4>
                                            <p className="text-gray-600">Price: <span className="font-semibold">${souvenir.price}</span></p>
                                            <p className="text-sm text-gray-500">{souvenir.description}</p>
                                        </div>
                                        <div className="flex items-center">
                                            <Dialog>
                                                <DialogTrigger asChild>
                                                    <Button
                                                        disabled={store.status === "Closed" || store.status === "Shut Down"}
                                                        className="bg-yellow-600 hover:bg-yellow-700 text-white font-semibold px-6 py-3 rounded-lg"
                                                    >
                                                        Purchase
                                                    </Button>
                                                </DialogTrigger>
                                                <DialogContent>
                                                    <DialogHeader>
                                                        <DialogTitle>Choose Amount</DialogTitle>
                                                        <DialogDescription>Enter the quantity to purchase.</DialogDescription>
                                                    </DialogHeader>

                                                    <div className="grid gap-4 py-4">
                                                        <div className="grid grid-cols-4 items-center gap-4">
                                                            <Label htmlFor="amount" className="text-right">
                                                                Amount
                                                            </Label>
                                                            <Input
                                                                id="amount"
                                                                type="number"
                                                                className="col-span-3"
                                                                onChange={(val) => calculatePrice(parseInt(val.target.value), souvenir.price)}
                                                            />
                                                        </div>
                                                    </div>

                                                    <DialogFooter className="w-full flex flex-col justify-between">
                                                        <h2 className="text-lg font-semibold">Total Price: ${price}</h2>
                                                        <Button
                                                            type="submit"
                                                            className="bg-yellow-700 hover:bg-yellow-800 text-white font-semibold px-6 py-3 rounded-lg"
                                                            onClick={() => purchaseSouvenir(souvenir.id)}
                                                            disabled={store.status === "Closed" || store.status === "Shut Down"}
                                                        >
                                                            Confirm Purchase
                                                        </Button>
                                                    </DialogFooter>
                                                </DialogContent>
                                            </Dialog>
                                        </div>
                                    </div>
                                ))
                            ) : (
                                <p className="text-gray-600">No souvenirs available.</p>
                            )}
                        </div>
                    </div>
                </div>
            ) : (
                <h1 className="text-2xl font-semibold text-gray-700">Store is {store?.status || "Unavailable"}</h1>
            )}
        </main>
    );
}
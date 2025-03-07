import {useStaffAuth} from "@/context/StaffAuthProvider.tsx";
import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {toast} from "sonner";
import {Store} from "@/ types/store.ts";
import {StoreTransaction} from "@/ types/store_transaction.ts";
import {Staff} from "@/ types/staff.ts";

export default function ViewStore() {
    const {staffId} = useStaffAuth();
    const [store, setStore] = useState<Store | null>(null);
    const [transactions, setTransactions] = useState<StoreTransaction[]>([]);

    const findStaffStore = async () => {
        try {
            invoke<Store>("find_staff_store", {selectedId: staffId}).then(setStore);
        } catch (e) {
            toast.error(`${e}`);
        }
    };

    const findStoreTransaction = async () => {
        try {
            invoke<StoreTransaction[]>("find_store_transaction", {selectedId: store?.id}).then(setTransactions);
        } catch (e) {
            toast.error(`${e}`);
        }
    };

    useEffect(() => {
        findStaffStore();
    }, []);

    useEffect(() => {
        if (store) {
            findStoreTransaction();
        }
    }, [store]);

    return (
        <div className="h-screen w-full bg-purple-200 flex flex-col p-16 overflow-auto gap-5">
            {store ? (
                <div className="w-full bg-white h-auto rounded-2xl shrink-0 flex flex-col p-8 gap-5">
                    <div className="w-full flex flex-row">
                        <div className="w-96 h-auto p-8 overflow-hidden">
                            <img className="object-contain w-full h-full rounded-lg" src={store.image_data} alt={store.name}/>
                        </div>
                        <div className="w-full h-auto flex flex-col p-8 gap-5">
                            <div className="w-full h-full justify-between flex flex-row">
                                <div className="w-auto flex flex-col gap-2">
                                    <h1 className="font-bold text-4xl">{store.name}</h1>
                                    <h4>{store.open_time} - {store.close_time}</h4>
                                    <h3>Status : {store.status}</h3>
                                </div>
                            </div>
                            <div>
                                <h1 className="font-bold text-2xl">Staffs</h1>
                                {store.staffs.length === 0 ? (
                                    <h2>None</h2>
                                ) : store.staffs.map((staff: Staff) => (
                                    <div key={staff.id}>
                                        <h2>{staff.name}</h2>
                                    </div>
                                ))}
                            </div>
                        </div>
                    </div>
                    <div>
                        <h1 className="font-bold text-2xl">Store Transactions</h1>
                        {transactions.length === 0 ? (
                            <h2>No transactions found</h2>
                        ) : (
                            <div className="w-full bg-gray-100 p-4 rounded-lg">
                                {transactions.map((transaction, index) => (
                                    <div key={index} className="border-b p-4">
                                        <p><strong>Customer ID:</strong> {transaction.customer_id}</p>
                                        <p><strong>Souvenir Name:</strong> {transaction.souvenir_name}</p>
                                        <p><strong>Count:</strong> {transaction.count}</p>
                                        <p><strong>Revenue:</strong> {transaction.value}</p>
                                    </div>
                                ))}
                            </div>
                        )}
                    </div>
                </div>
            ) : (
                <h1>You are not assigned to any Store</h1>
            )}
        </div>
    );
}

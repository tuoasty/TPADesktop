import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {toast} from "sonner";
import {StoreRevenue} from "@/ types/store_revenue.ts";

export default function ViewStoreRevenue() {
    const [storeRevenue, setStoreRevenue] = useState<StoreRevenue[]>([]);

    const findStoreRevenue = async () => {
        try {
            invoke<StoreRevenue[]>("find_store_revenue").then(setStoreRevenue);
        } catch (e) {
            toast.error(`${e}`);
        }
    };

    useEffect(() => {
        findStoreRevenue();
    }, []);

    const totalRevenue = storeRevenue.reduce((sum, transaction) => sum + transaction.value, 0)

    return (
        <div className="h-screen w-full bg-purple-200 flex justify-center items-center p-8">
            <div className="w-3/5 bg-white h-auto rounded-2xl flex flex-col p-8 gap-5 shadow-lg">
                <h1 className="font-bold text-4xl text-center">Store Revenue Transactions</h1>
                {storeRevenue.length > 0 ? (
                    <table className="w-full border-collapse border border-gray-300">
                        <thead>
                        <tr className="bg-gray-200">
                            <th className="border border-gray-300 px-4 py-2">ID</th>
                            <th className="border border-gray-300 px-4 py-2">Store ID</th>
                            <th className="border border-gray-300 px-4 py-2">Souvenir ID</th>
                            <th className="border border-gray-300 px-4 py-2">Time</th>
                            <th className="border border-gray-300 px-4 py-2">Value</th>
                        </tr>
                        </thead>
                        <tbody>
                        {storeRevenue.map((transaction) => (
                            <tr key={transaction.id} className="text-center">
                                <td className="border border-gray-300 px-4 py-2">{transaction.id}</td>
                                <td className="border border-gray-300 px-4 py-2">{transaction.store_id}</td>
                                <td className="border border-gray-300 px-4 py-2">{transaction.souvenir_id}</td>
                                <td className="border border-gray-300 px-4 py-2">{transaction.time}</td>
                                <td className="border border-gray-300 px-4 py-2">{transaction.value}</td>
                            </tr>
                        ))}
                        </tbody>
                        <tfoot>
                        <tr className="bg-gray-200 font-bold">
                            <td className="border border-gray-300 px-4 py-2 text-center" colSpan={4}>Total Revenue</td>
                            <td className="border border-gray-300 px-4 py-2 text-center">{totalRevenue}</td>
                        </tr>
                        </tfoot>
                    </table>
                ) : (
                    <h1 className="text-center text-xl">No store transactions made yet</h1>
                )}
            </div>
        </div>
    );
}

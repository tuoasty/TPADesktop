import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {toast} from "sonner";
import {RideRevenue} from "@/ types/ride_revenue.ts";

export default function ViewRideRevenue() {
    const [rideRevenue, setRideRevenue] = useState<RideRevenue[]>([]);

    const findRideRevenue = async () => {
        try {
            invoke<RideRevenue[]>("find_ride_revenue").then(setRideRevenue);
        } catch (e) {
            toast.error(`${e}`);
        }
    };

    useEffect(() => {
        findRideRevenue();
    }, []);

    const totalRevenue = rideRevenue.reduce((sum, transaction) => sum + transaction.value, 0)

    return (
        <div className="h-screen w-full bg-purple-200 flex justify-center items-center p-8">
            <div className="w-3/5 bg-white h-auto rounded-2xl flex flex-col p-8 gap-5 shadow-lg">
                <h1 className="font-bold text-4xl text-center">Ride Revenue Transactions</h1>
                {rideRevenue.length > 0 ? (
                    <table className="w-full border-collapse border border-gray-300">
                        <thead>
                        <tr className="bg-gray-200">
                            <th className="border border-gray-300 px-4 py-2">ID</th>
                            <th className="border border-gray-300 px-4 py-2">Ride ID</th>
                            <th className="border border-gray-300 px-4 py-2">Time</th>
                            <th className="border border-gray-300 px-4 py-2">Value</th>
                        </tr>
                        </thead>
                        <tbody>
                        {rideRevenue.map((transaction) => (
                            <tr key={transaction.id} className="text-center">
                                <td className="border border-gray-300 px-4 py-2">{transaction.id}</td>
                                <td className="border border-gray-300 px-4 py-2">{transaction.ride_id}</td>
                                <td className="border border-gray-300 px-4 py-2">{transaction.time}</td>
                                <td className="border border-gray-300 px-4 py-2">{transaction.value}</td>
                            </tr>
                        ))}
                        </tbody>
                        <tfoot>
                        <tr className="bg-gray-200 font-bold">
                            <td className="border border-gray-300 px-4 py-2 text-center" colSpan={3}>Total Revenue</td>
                            <td className="border border-gray-300 px-4 py-2 text-center">{totalRevenue}</td>
                        </tr>
                        </tfoot>
                    </table>
                ) : (
                    <h1 className="text-center text-xl">No ride transactions made yet</h1>
                )}
            </div>
        </div>
    );
}

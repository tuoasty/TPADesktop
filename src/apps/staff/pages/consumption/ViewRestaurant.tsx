import { useEffect, useState } from "react";
import { useStaffAuth } from "@/context/StaffAuthProvider.tsx";
import { invoke } from "@tauri-apps/api/core";
import { toast } from "sonner";
import {RestaurantOrder} from "@/ types/restaurant_order.ts";
import {Restaurant} from "@/ types/restaurant.ts";
import {Staff} from "@/ types/staff.ts";
import {Menu} from "@/ types/menu.ts";
export default function ViewRestaurant() {
    const [restaurant, setRestaurant] = useState<Restaurant | null>(null);
    const { staffId , role} = useStaffAuth();
    const [orders, setOrders] = useState<RestaurantOrder[]>([]);

    const findStaffRestaurant = async () => {
        try {
            const result = await invoke<Restaurant>("find_staff_restaurant", { selectedId: staffId });
            setRestaurant(result);
        } catch (e) {
            toast.error("Error fetching restaurant data.");
            setRestaurant(null);
        }
    };

    const findRestaurantOrders = async () => {
        try {
            invoke<RestaurantOrder[]>("find_restaurant_orders", {selectedId:restaurant?.id}).then(setOrders)
        } catch (e) {
            toast.error(`${e}`)
        }
    }

    useEffect(() => {
        findStaffRestaurant();
    }, []);

    useEffect(() => {
        findRestaurantOrders();
    }, [restaurant]);

    const takeOrder = async (orderId: number) => {
        try {
            await invoke("set_order_status", {selectedId:orderId, newStatus:"Ongoing Order"})
            toast.success("Successfully took order")
            findRestaurantOrders();
        } catch (e) {
            toast.error(`${e}`)
        }
    };

    const finishOrder = async (orderId: number) => {
        try {
            await invoke("set_order_status", {selectedId:orderId, newStatus:"Finished"})
            toast.success("Successfully finished order")
            findRestaurantOrders();
        } catch (e) {
            toast.error(`${e}`)
        }
    };

    const markOrderAsReady = async (orderId: number) => {
        try {
            await invoke("set_order_status", {selectedId:orderId, newStatus:"Ready to Serve"})
            toast.success("Successfully marked order as ready")
            findRestaurantOrders();
        } catch (e) {
            toast.error(`${e}`)
        }
    };

    if (!restaurant) {
        return (
            <div className="h-screen w-full bg-purple-200 flex flex-col items-center justify-center p-16">
                <h1 className="text-2xl font-bold text-gray-800">Staff is not assigned.</h1>
            </div>
        );
    }

    return (
        <div className="h-screen w-full bg-purple-200 flex flex-col p-16 overflow-auto gap-5">
            <div key={restaurant.id} className="w-full bg-white rounded-2xl shadow-lg p-6 flex flex-col md:flex-row">
                <div className="w-full md:w-96 h-72 md:h-full p-4 overflow-hidden">
                    <img
                        className="object-cover w-full h-full rounded-lg shadow-md"
                        src={restaurant.image_data}
                        alt={restaurant.name}
                    />
                </div>

                <div className="w-full flex flex-col p-8 gap-4">
                    <div className="flex flex-col md:flex-row justify-between items-start">
                        <div>
                            <h1 className="font-bold text-4xl">{restaurant.name}</h1>
                            <h2 className="text-xl text-gray-600">Cuisine: {restaurant.cuisine}</h2>
                            <h4 className="text-gray-700">Open Time: {restaurant.open_time} - {restaurant.close_time}</h4>
                            <h3 className={`text-lg font-semibold ${restaurant.status === "Open" ? "text-green-600" : "text-red-600"}`}>
                                Status: {restaurant.status}
                            </h3>
                        </div>
                    </div>

                    <div>
                        <h1 className="font-bold text-2xl mb-2">Staff Members</h1>
                        {restaurant.staffs.length === 0 ? (
                            <h2 className="text-gray-500">None</h2>
                        ) : (
                            <div className="grid grid-cols-1 md:grid-cols-2 gap-2">
                                {restaurant.staffs.map((staff: Staff) => (
                                    <div key={staff.id} className="bg-purple-100 p-2 rounded-lg">
                                        <h2 className="font-medium">{staff.role}: {staff.name}</h2>
                                    </div>
                                ))}
                            </div>
                        )}
                    </div>

                    <div>
                        <h1 className="font-bold text-2xl mb-2">Menu</h1>
                        {restaurant.menus.length === 0 ? (
                            <h2 className="text-gray-500">No menu items available.</h2>
                        ) : (
                            restaurant.menus.map((menu: Menu) => (
                                <div key={menu.id} className="bg-purple-100 rounded-2xl p-4 flex justify-between items-center mb-2 shadow-md">
                                    <div className="flex items-center">
                                        <div className="h-20 w-20 mr-4 overflow-hidden">
                                            <img className="object-cover w-full h-full rounded-lg"
                                                 src={menu.image_data}
                                                 alt={menu.name}
                                            />
                                        </div>
                                        <div>
                                            <h4 className="font-bold text-lg">{menu.name}</h4>
                                            <h4 className="text-gray-600">Price: ${menu.price}</h4>
                                        </div>
                                    </div>
                                </div>
                            ))
                        )}
                    </div>
                </div>
            </div>

            <div className="w-full bg-white rounded-2xl shadow-lg p-6 mt-6">
                <h1 className="font-bold text-2xl mb-4">Orders</h1>
                {orders.length === 0 ? (
                    <h2 className="text-gray-500">No orders yet.</h2>
                ) : (
                    orders.map((order) => (
                        order.status != "Finished" && (
                            <div key={order.id} className="bg-purple-100 rounded-2xl p-4 flex justify-between items-center mb-2 shadow-md">
                                <div>
                                    <h4 className="font-bold">Customer ID: {order.customer_id}</h4>
                                    <h4>Menu: {order.menu_name}</h4>
                                    <h4>Status: {order.status}</h4>
                                </div>
                                <div className="flex gap-4">
                                    {role === "Waiter" && (
                                        <>
                                            <button
                                                className="bg-blue-500 text-white px-4 py-2 rounded disabled:opacity-50"
                                                onClick={() => takeOrder(order.id)}
                                                disabled={order.status !== "New Order"}
                                            >
                                                Take Order
                                            </button>
                                            <button
                                                className="bg-green-500 text-white px-4 py-2 rounded disabled:opacity-50"
                                                onClick={() => finishOrder(order.id)}
                                                disabled={order.status !== "Ready to Serve"}
                                            >
                                                Finish Order
                                            </button>
                                        </>
                                    )}
                                    {role === "Chef" && (
                                        <button
                                            className="bg-yellow-500 text-white px-4 py-2 rounded disabled:opacity-50"
                                            onClick={() => markOrderAsReady(order.id)}
                                            disabled={order.status !== "Ongoing Order"}
                                        >
                                            Mark as Ready
                                        </button>
                                    )}
                                </div>
                            </div>
                        )
                    ))
                )}
            </div>
        </div>
    );
}

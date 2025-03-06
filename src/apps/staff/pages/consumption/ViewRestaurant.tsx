import { useEffect, useState } from "react";
import { useStaffAuth } from "@/context/StaffAuthProvider.tsx";
import { invoke } from "@tauri-apps/api/core";
import { toast } from "sonner";
import {Restaurant} from "@/ types/restaurant.ts";
import {Staff} from "@/ types/staff.ts";
import {Menu} from "@/ types/menu.ts";

export default function ViewRestaurant() {
    const [restaurant, setRestaurant] = useState<Restaurant | null>(null);
    const { staffId } = useStaffAuth();

    const findStaffRestaurant = async () => {
        try {
            const result = await invoke<Restaurant>("find_staff_restaurant", { selectedId: staffId });
            setRestaurant(result);
        } catch (e) {
            toast.error("Error fetching restaurant data.");
            setRestaurant(null);
        }
    };

    useEffect(() => {
        findStaffRestaurant();
    }, []);

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
        </div>
    );
}

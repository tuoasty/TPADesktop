import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { toast } from "sonner";
import { useCustomerAuth } from "@/context/CustomerAuthProvider.tsx";
import { useNavigate } from "react-router-dom";
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
import {Restaurant} from "@/ types/restaurant.ts";
import {Menu} from "@/ types/menu.ts";

interface Props {
    restaurantId: number;
}

export default function RestaurantDashboard({ restaurantId }: Props) {
    const [restaurant, setRestaurant] = useState<Restaurant | null>(null);
    const [orderCount, setOrderCount] = useState(1);
    const [selectedMenu, setSelectedMenu] = useState<Menu | null>(null);
    const navigate = useNavigate();

    const { customerId, name, balance, customerIsLoggedIn } = useCustomerAuth();

    const fetchRestaurant = async () => {
        try {
            invoke<Restaurant>("find_restaurant_by_id", { selectedId: restaurantId }).then(setRestaurant);
        } catch (e) {
            toast.error(`${e}`);
        }
    };

    useEffect(() => {
        fetchRestaurant();
    }, []);

    const orderRestaurantFood = async () => {
        if (!(await customerIsLoggedIn())) {
            toast.error("You must be logged in to purchase");
            navigate("login");
            return;
        }
        try {
            await invoke("order_restaurant_food", {selectedRestaurantId:restaurant?.id, selectedMenuId:selectedMenu?.id, selectedCustomerId:customerId, orderCount:orderCount})
            toast.success("Successfully ordered food")
        } catch (e) {
            toast.error(`${e}`)
        }
    };

    return (
        <main className="bg-green-200 min-h-screen flex flex-col items-center justify-center p-6">
            {restaurant && restaurant.status === "Open" ? (
                <div className="bg-white shadow-lg rounded-xl overflow-hidden w-full max-w-5xl flex flex-col md:flex-row p-6 gap-6">
                    <div className="w-full md:w-1/3">
                        <img className="object-cover w-full h-64 rounded-lg" src={restaurant.image_data} alt={restaurant.name} />
                    </div>
                    <div className="w-full md:w-2/3 flex flex-col gap-4">
                        {name && (
                            <div className="text-gray-800">
                                <h1 className="text-lg font-semibold">Welcome, {name}!</h1>
                                <p className="text-gray-600">Balance: <span className="font-bold">{balance}</span></p>
                            </div>
                        )}

                        <h1 className="text-3xl font-bold text-gray-900">{restaurant.name}</h1>
                        <div className="space-y-6">
                            {restaurant.menus.length > 0 ? (
                                restaurant.menus.map((menu: Menu) => (
                                    <div key={menu.id} className="bg-gray-50 rounded-xl p-4 flex flex-col md:flex-row gap-6 shadow">
                                        <div className="w-full md:w-32 h-32 overflow-hidden">
                                            <img className="object-cover w-full h-full rounded-lg" src={menu.image_data} alt={menu.name} />
                                        </div>
                                        <div className="flex-1">
                                            <h4 className="font-bold text-lg">{menu.name}</h4>
                                            <p className="text-gray-600">Price: <span className="font-semibold">{menu.price}</span></p>
                                        </div>
                                        <div className="flex items-center">
                                            <Dialog>
                                                <DialogTrigger asChild>
                                                    <Button
                                                        className="bg-green-600 hover:bg-green-700 text-white font-semibold px-6 py-3 rounded-lg"
                                                        onClick={() => setSelectedMenu(menu)}
                                                    >
                                                        Order
                                                    </Button>
                                                </DialogTrigger>
                                                <DialogContent>
                                                    <DialogHeader>
                                                        <DialogTitle>Order Food</DialogTitle>
                                                        <DialogDescription>Select the quantity for your order.</DialogDescription>
                                                    </DialogHeader>

                                                    <div className="grid gap-4 py-4">
                                                        <div className="grid grid-cols-4 items-center gap-4">
                                                            <Label htmlFor="orderCount" className="text-right">
                                                                Quantity
                                                            </Label>
                                                            <Input
                                                                id="orderCount"
                                                                type="number"
                                                                min="1"
                                                                className="col-span-3"
                                                                value={orderCount}
                                                                onChange={(e) => setOrderCount(parseInt(e.target.value))}
                                                            />
                                                        </div>
                                                    </div>

                                                    <DialogFooter className="w-full flex flex-col justify-between">
                                                        <h2 className="text-lg font-semibold">Total Price: {selectedMenu ? selectedMenu.price * orderCount : 0}</h2>
                                                        <Button
                                                            className="bg-green-700 hover:bg-green-800 text-white font-semibold px-6 py-3 rounded-lg"
                                                            onClick={orderRestaurantFood}
                                                        >
                                                            Confirm Order
                                                        </Button>
                                                    </DialogFooter>
                                                </DialogContent>
                                            </Dialog>
                                        </div>
                                    </div>
                                ))
                            ) : (
                                <p className="text-gray-600">No menu items available.</p>
                            )}
                        </div>
                    </div>
                </div>
            ) : (
                <h1 className="text-2xl font-semibold text-gray-700">Restaurant is {restaurant?.status || "Unavailable"}</h1>
            )}
        </main>
    );
}
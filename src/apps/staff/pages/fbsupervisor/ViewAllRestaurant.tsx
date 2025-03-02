import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {Restaurant} from "@/ types/restaurant.ts";
import {Menu} from "@/ types/menu.ts";
import {
    Dialog,
    DialogContent,
    DialogDescription, DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger
} from "@/components/ui/dialog.tsx";
import {Button} from "@/components/ui/button.tsx";
import {toast} from "sonner";
import {Staff} from "@/ types/staff.ts";
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from "@/components/ui/select.tsx";

export default function ViewAllRestaurant() {
    const [restaurants, setRestaurants] = useState<Restaurant[]>([]);
    const [staffs, setStaffs] = useState<Staff[]>([]);
    const [selectedId, setSelectedId] = useState<number | null>(null);

    const fetchRestaurants = async () => {
        invoke<Restaurant[]>("find_all_restaurant")
            .then(setRestaurants)
    }

    const fetchConsumptionStaffs = async () => {
        invoke<Staff[]>("find_all_consumption_staff")
            .then(setStaffs)
    }

    useEffect(() => {
        fetchRestaurants();
        fetchConsumptionStaffs();
    }, []);

    const changeRestaurantStatus = async (id:number, status:string) => {
        try {
            await invoke("change_restaurant_status", {restaurantId:id, restaurantStatus:status})
            toast.success("Successfully updated restaurant status");
            fetchRestaurants();
        } catch (e) {
            toast.error(`${e}`)
        }
    }

    const assignStaffToRestaurant = async (restaurantId:number) => {
        try {
            await invoke("assign_staff_to_restaurant", {staffId:selectedId, restaurantId:restaurantId})
            toast.success("Successfully assigned staff");
            fetchRestaurants();
        } catch (e) {
            toast.error(`${e}`)
        }
    }

    return (
        <div className="h-screen w-full bg-purple-200 flex flex-col p-16 overflow-auto gap-5">
            {restaurants.length > 0 && (
                restaurants.map((restaurant: Restaurant) => (
                    <div key={restaurant.id} className="w-full bg-white h-min-72 rounded-2xl shrink-0 flex">
                        <div className="w-96 h-full p-8 overflow-hidden">
                            <img className="object-contain w-full h-full rounded-lg" src={restaurant.image_data}
                                 alt={restaurant.name}/>
                        </div>
                        <div className="w-full h-full flex flex-col p-8 gap-2">
                            <div className="w-full h-auto flex flex-row justify-between gap-4">
                                <div>
                                    <h1 className="font-bold text-4xl">{restaurant.name}</h1>
                                    <h2 className="text-2xl">Cuisine : {restaurant.cuisine}</h2>
                                    <h4>Open Time : {restaurant.open_time} - {restaurant.close_time}</h4>
                                    <h3>Status : {restaurant.status}</h3>
                                </div>
                                <div className="w-48 h-auto flex flex-col gap-4 mb-4">
                                    <Dialog>
                                        <DialogTrigger asChild>
                                            <Button className="bg-purple-700 w-48 h-12">Assign Staff</Button>
                                        </DialogTrigger>
                                        <DialogContent>
                                            <DialogHeader>
                                                <DialogTitle>Restaurant Staff</DialogTitle>
                                                <DialogDescription>Choose staff to assign.
                                                    Restaurants need two Chefs and Waiters</DialogDescription>
                                            </DialogHeader>
                                            <Select onValueChange={(val) => setSelectedId(Number(val))}>
                                                <SelectTrigger>
                                                    <SelectValue placeholder="Restaurant Staff"/>
                                                </SelectTrigger>
                                                <SelectContent>
                                                    {staffs.length > 0 && (
                                                        staffs.map((staff: Staff) => (
                                                            <SelectItem key={staff.id} value={staff.id.toString()}>{staff.role} : {staff.name}</SelectItem>
                                                        ))
                                                    )}
                                                </SelectContent>
                                            </Select>
                                            <DialogFooter>
                                                <Button onClick={() => assignStaffToRestaurant(restaurant.id)} type="submit" className="bg-purple-700">Confirm</Button>
                                            </DialogFooter>
                                        </DialogContent>
                                    </Dialog>
                                    <Button className={`w-48 h-12 ${restaurant.status == "Closed" ? "bg-green-500" : "bg-red-500"}`}
                                    onClick={() => {changeRestaurantStatus(restaurant.id, restaurant.status)}}>
                                        {restaurant.status == "Closed" ? "Open" : "Close"}
                                    </Button>
                                </div>
                            </div>
                            <div>
                                <h1 className="font-bold text-2xl">Staffs</h1>
                                {restaurant.staffs.length == 0 ? (
                                    <h2>None</h2>
                                ) : restaurant.staffs.map((staff: Staff) => (
                                    <div key={staff.id}>
                                        <h2>{staff.role} : {staff.name}</h2>
                                    </div>
                                ))}
                            </div>
                            {restaurant.menus.length > 0 && (
                                restaurant.menus.map((menu: Menu) => (
                                    <div key={menu.id} className="bg-purple-200 rounded-2xl p-2 flex justify-between">
                                        <div className="flex flex-row">
                                            <div className="h-28 w-28 mr-4 overflow-hidden">
                                                <img className="object-cover w-full h-full rounded-lg"
                                                     src={menu.image_data}
                                                     alt={menu.name}/>
                                            </div>
                                            <div className="flex justify-center flex-col">
                                                <h4 className="font-bold">{menu.name}</h4>
                                                <h4>Price : {menu.price}</h4>
                                            </div>
                                        </div>
                                        <div className="flex justify-center place-items-center mr-6">
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
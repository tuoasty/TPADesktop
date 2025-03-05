import {useEffect, useState} from "react";
import {Store} from "@/ types/store.ts";
import {invoke} from "@tauri-apps/api/core";
import {toast} from "sonner";
import {useCustomerAuth} from "@/context/CustomerAuthProvider.tsx";
import {useNavigate} from "react-router-dom";
import {Restaurant} from "@/ types/restaurant.ts";
import {Menu} from "@/ types/menu.ts";

interface Props {
    restaurantId:number;
}
export default function RestaurantDashboard(p:Props){
    const [restaurant, setRestaurant] = useState<Restaurant | null>(null);
    const navigate = useNavigate();

    const {name, balance, customerIsLoggedIn} = useCustomerAuth();

    // const purchaseSouvenir = async (souvenirId:number) => {
    //     if (!await customerIsLoggedIn()) {
    //         toast.error("You must be logged in to purchase")
    //         navigate("login")
    //         return;
    //     }
    //
    //     try {
    //         await invoke("purchase_souvenir", {souvenirId:souvenirId, souvenirCount:count});
    //         toast.success("Successfully purchased item");
    //     } catch (e) {
    //         toast.error(`${e}`)
    //     }
    // }

    const fetchRestaurant = async () => {
        try {
            invoke<Store>("find_restaurant_by_id", {selectedId:p.restaurantId}).then(setRestaurant);
        } catch (e) {
            toast.error(`${e}`)
        }
    }

    useEffect(() => {
        fetchRestaurant();
    }, []);

    return (
        <main className="bg-green-200 h-full w-full flex flex-col place-items-center justify-center">
            {restaurant && restaurant.status == "Open" ? (
                <div className="h-full w-full flex">
                    <div className="w-[25%] h-auto">
                        <img className="object-contain w-full h-full rounded-lg" src={restaurant.image_data}
                             alt={restaurant.name}/>
                    </div>
                    <div className="w-full p-12 gap-5">
                        {name && (
                            <div>
                                <h1>Welcome {name}</h1>
                                <h1>Balance : {balance}</h1>
                            </div>
                        )}
                        <h1 className="text-2xl font-bold">{restaurant.name}</h1>
                        {restaurant.menus.length > 0 && (
                            restaurant.menus.map((menu:Menu) => (
                                <div className="w-auto gap-5 m-12">
                                    <div key={menu.id}
                                         className="bg-white rounded-2xl p-2 flex justify-between w-full gap-5">
                                        <div className="flex flex-row w-full">
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
                                        <div className="flex flex-col place-items-center justify-center p-8">
                                        {/*    Buttons*/}
                                        </div>
                                    </div>
                                </div>
                            ))
                        )}
                    </div>
                </div>
            ) : (
                <h1>Restaurant is {restaurant?.status}</h1>
            )}
        </main>
    )
}
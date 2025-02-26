import {Input} from "@/components/ui/input.tsx";
import {Label} from "@/components/ui/label.tsx";
import {Button} from "@/components/ui/button.tsx";

export default function AddNewMenu(){
    return (
        <div className="h-full w-full bg-purple-200 flex justify-center place-items-center">
            <form className="bg-white h-[50%] w-[25%] flex flex-col justify-center place-items-center gap-5 p-12 rounded-2xl">
                <h1 className="font-bold text-2xl">Add New Menu</h1>
                <Input type="text" placeholder="Menu Name"/>
                <Input type="number" placeholder="Menu Price"/>
                <div className="grid w-full max-w-sm items-center gap-1.5">
                    <Label htmlFor="picture">Picture</Label>
                    <Input type="file" id="picture"/>
                </div>
                <Button className="bg-purple-500 w-full">Add Menu</Button>
            </form>
        </div>
    )
}
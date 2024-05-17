import { TodoItem } from "../App";
import { Button } from '@/components/ui/button';
import { useToast } from "@/components/ui/use-toast"
import { Checkbox } from "@/components/ui/checkbox"
import { invoke } from "@tauri-apps/api";
import { Trash } from 'lucide-react';
import {
    Tooltip,
    TooltipContent,
    TooltipProvider,
    TooltipTrigger,
} from "@/components/ui/tooltip"
import {
    AlertDialog,
    AlertDialogAction,
    AlertDialogCancel,
    AlertDialogContent,
    AlertDialogDescription,
    AlertDialogFooter,
    AlertDialogHeader,
    AlertDialogTitle,
    AlertDialogTrigger,
} from "@/components/ui/alert-dialog"
import clsx from "clsx";


interface TodoItemProps {
    item: TodoItem;
    reloadList: () => void;
}
export function TodoItemRecord(props: TodoItemProps) {
    const { item, reloadList } = props
    const { toast } = useToast()
    const time = new Date(item.datetime).toLocaleString()

    const deleteItem = async () => {
        const result = await invoke("delete_item_by_id", { id: item.id })
        if (result) {
            toast({ description: "Delete item" })
            reloadList()
        }
    }

    const switchItemStatue = async (newStatue: boolean) => {
        const result = await invoke("switch_item_status", { id: item.id, isDone: newStatue })
        if (result) {
            reloadList()
        }
    }

    return <div className="flex justify-between items-center my-2 mx-auto max-w-xl">
        <span className={clsx("w-4 text-[14px] text-gray-400", item.done && " italic")}>{item.id}</span>
        <Checkbox checked={item.done} onCheckedChange={async (checked) => {
            await switchItemStatue(Boolean(checked));
        }} id={`item_${item.id}`} className=" mx-4" />

        <TooltipProvider>
            <Tooltip>
                <TooltipTrigger asChild>
                    <label htmlFor={`item_${item.id}`} className=" flex-1 text-left truncate">
                        <span className={clsx(item.done && " opacity-60 line-through")}>{item.title}</span>
                    </label>
                </TooltipTrigger>
                <TooltipContent >
                    <span>{item.title}</span>
                </TooltipContent>
            </Tooltip>
            <Tooltip>
                <TooltipTrigger asChild>
                    <span className="inline-block mx-4">{time.split(' ')[0]}</span>
                </TooltipTrigger>
                <TooltipContent>
                    <span>{time}</span>
                </TooltipContent>
            </Tooltip>
        </TooltipProvider>
        <AlertDialog>
            <AlertDialogTrigger asChild>
                <Button size="sm" variant="ghost" className=" hover:bg-red-600 hover:text-white"><Trash size={16} /></Button>
            </AlertDialogTrigger>
            <AlertDialogContent>
                <AlertDialogHeader>
                    <AlertDialogTitle>Are you absolutely sure delete item?</AlertDialogTitle>
                    <AlertDialogDescription>
                        This action cannot be undone.
                    </AlertDialogDescription>
                </AlertDialogHeader>
                <AlertDialogFooter>
                    <AlertDialogCancel>Cancel</AlertDialogCancel>
                    <AlertDialogAction onClick={deleteItem}>Confirm</AlertDialogAction>
                </AlertDialogFooter>
            </AlertDialogContent>
        </AlertDialog>
    </div>
}
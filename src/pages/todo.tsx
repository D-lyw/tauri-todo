import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { TodoItemRecord } from "../components/todo-item";
import { Button } from "@/components/ui/button";
import { Input } from '@/components/ui/input';
import { Plus } from 'lucide-react'
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { ScrollArea } from "@/components/ui/scroll-area"

export interface TodoItem {
    id: number;
    title: string;
    done: boolean;
    datetime: string;
}

const TodoPage = () => {
    const [todoItem, setTotoItem] = useState<string>("");

    const [todoList, setTotoList] = useState<TodoItem[]>([]);
    const [page, setPage] = useState<number>(1);

    useEffect(() => {
        queryList()
    }, [])

    async function queryList() {
        const list = await invoke<TodoItem[]>('query_list_by_page', { page: page })
        setTotoList(page == 1 ? list : todoList.concat(list))
    }

    async function addTodoItem() {
        const result = await invoke<boolean>('add_todo_item', { title: todoItem })
        console.log(result);
        if (result) {
            setTotoItem("");
        }
    }
    
    return <div className=" overflow-hidden">
        <div className="flex w-full max-w-sm items-center space-x-2 mt-16 mb-4 mx-auto">
            <Input value={todoItem} onChange={(e) => setTotoItem(e.currentTarget.value)} type="text" placeholder="Todo ..." />
            <Button type="submit" onClick={async () => {
                await addTodoItem();
                await queryList();
            }}><Plus /></Button>
        </div>

        <Tabs defaultValue="active" className=" text-center">
            <TabsList>
                <TabsTrigger value="active">Active</TabsTrigger>
                <TabsTrigger value="done">Done</TabsTrigger>
                <TabsTrigger value="all">All</TabsTrigger>
            </TabsList>
            <ScrollArea className=" h-[380px] w-min m-auto pt-0">

                <TabsContent value="active">
                    {
                        todoList.filter((item) => !item.done).map((item, index) => {
                            return <TodoItemRecord item={item} key={index} reloadList={() => {
                                setPage(1);
                                queryList();
                            }} />
                        })
                    }
                </TabsContent>
                <TabsContent value="done">
                    {
                        todoList.filter((item) => item.done).map((item, index) => {
                            return <TodoItemRecord item={item} key={index} reloadList={() => {
                                setPage(1);
                                queryList();
                            }} />
                        })
                    }
                </TabsContent>
                <TabsContent value="all">
                    {
                        todoList.map((item, index) => {
                            return <TodoItemRecord item={item} key={index} reloadList={() => {
                                setPage(1);
                                queryList();
                            }} />
                        })
                    }
                </TabsContent>
            </ScrollArea>

        </Tabs>
    </div>
}

export default TodoPage;


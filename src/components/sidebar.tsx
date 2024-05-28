import clsx from 'clsx';
import { ListTodo, Settings, WalletMinimal } from 'lucide-react';
import { NavLink } from 'react-router-dom';
import { WebviewWindow } from '@tauri-apps/api/window';
import { Button } from '@/components/ui/button';

const Sidebar = () => {
    return <div className=" relative h-full p-3 pt-12">
        <div className='grid grid-cols-1 gap-y-2'>
            <NavLink to="/" className={({ isActive }) => {
                return clsx("flex items-center gap-2 rounded-xl px-3 py-2 text-lg  hover:text-foreground", !isActive && "text-muted-foreground", isActive && " bg-gray-50/60 shadow text-foreground");
            }}>
                <ListTodo className='h-5 w-5' />
                Todos
            </NavLink>
            <NavLink to="/web3" className={({ isActive }) => {
                return clsx("flex items-center gap-2 rounded-xl px-3 py-2 text-lg hover:text-foreground", !isActive && "text-muted-foreground", isActive && " bg-gray-50/60 shadow text-foreground");
            }}>
                <WalletMinimal className='h-5 w-5' />
                Crypto
            </NavLink>
        </div>
        <div className='flex-shrink-0 fixed bottom-3'>
            <Button variant="ghost" size="icon" onClick={() => {
                new WebviewWindow('Setting', {
                    url: '/setting',
                    resizable: false,
                    focus: true,
                    center: true,
                    width: 700,
                    height: 500,
                    theme: "light",
                    title: "Setting"
                });
            }}>
                <Settings />
            </Button>
        </div>
    </div>
}

export default Sidebar;
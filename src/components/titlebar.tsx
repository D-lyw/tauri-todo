import { PanelRight, Pin, PinOff } from 'lucide-react';
import {
    Tooltip,
    TooltipContent,
    TooltipProvider,
    TooltipTrigger,
} from "@/components/ui/tooltip"
import { useState } from 'react';
import { appWindow } from '@tauri-apps/api/window';

interface TitleBarPorps {
    sidebarIsOpen: boolean;
    setSidebarOpen: (isOpen: boolean) => void;
}

const TitleBar = (props: TitleBarPorps) => {
    const { sidebarIsOpen, setSidebarOpen } = props;

    const [isTopWindow, setTopWindow] = useState<boolean>(false);

    const toggleWindowTop = async () => {
        await appWindow.setAlwaysOnTop(!isTopWindow);
        setTopWindow(!isTopWindow);
    }

    return <div data-tauri-drag-region className=' w-full flex justify-between pr-1'>
        <TooltipProvider>
            <Tooltip>
                <TooltipTrigger asChild>
                    <div className=' p-[3px] hover:bg-gray-200/50 rounded-sm'>
                        <PanelRight className=" rotate-180" size="16" onClick={() => setSidebarOpen(!sidebarIsOpen)} />
                    </div>
                </TooltipTrigger>
                <TooltipContent>
                    Toggle Sidebar
                </TooltipContent>
            </Tooltip>
        </TooltipProvider>

        <TooltipProvider>
            <Tooltip>
                <TooltipTrigger asChild>
                    <div className=' p-[3px] hover:bg-gray-200/50 rounded-sm' onClick={toggleWindowTop}>
                        {isTopWindow ? <PinOff size="16" /> : <Pin size="16" />}
                    </div>
                </TooltipTrigger>
                <TooltipContent>
                    Toggle Window Toppest
                </TooltipContent>
            </Tooltip>
        </TooltipProvider>
    </div>
}

export default TitleBar;
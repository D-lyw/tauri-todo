import { Toaster } from "@/components/ui/toaster"
import { useState } from "react";
import TitleBar from "./components/titlebar";
import Sidebar from "./components/sidebar";
import { Outlet } from "react-router-dom";

export function MainLayout() {
    const [sidebarIsOpen, setSidebarOpen] = useState<boolean>(true);

    return <div className=" h-screen w-screen relative flex items-start bg-gray-50">
        <Toaster />
        <div className=" fixed z-50 top-0 title-bar flex items-center h-7 w-full pl-[72px]" data-tauri-drag-region>
            <TitleBar sidebarIsOpen={sidebarIsOpen} setSidebarOpen={setSidebarOpen} />
        </div>

        {/* Sidebar */}
        {sidebarIsOpen && <div className=" w-56 h-screen shrink-0 bg-gray-200">
            <Sidebar />
        </div>}
        {/* Content */}
        <div className=" h-screen grow">
            <Outlet />
        </div>
    </div>
}
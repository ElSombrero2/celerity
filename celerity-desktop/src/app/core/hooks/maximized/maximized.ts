import { appWindow } from "@tauri-apps/api/window";
import { useEffect, useState } from "react";


export function useMaximized() {
    const [maximzed, setMaximized] = useState(false)
    
    useEffect(() => {
        appWindow.onResized(async () => setMaximized(await appWindow.isMaximized()))
    }, []);

    return maximzed 
}
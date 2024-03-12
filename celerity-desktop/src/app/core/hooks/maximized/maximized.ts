import { appWindow } from "@tauri-apps/api/window";
import { useEffect, useState } from "react";

export function useMaximized() {
    const [maximized, setMaximized] = useState(false)

    useEffect(() => {
        appWindow.isMaximized().then((maximized) => setMaximized(maximized))
        appWindow.onResized(async () => setMaximized(await appWindow.isMaximized()))
    }, []);

    return maximized 
}
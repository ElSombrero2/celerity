import { appWindow } from "@tauri-apps/api/window"
import { Button } from "./Button/Button"
import { useMaximized } from "../../../../../app/core/hooks/maximized/maximized"

export const Buttons = () => {
    const maximized = useMaximized()
    return (
        <div className={`buttons ${maximized && 'maximized'}`}>
            <Button onClick={() => appWindow.minimize() }>
                <i className="fa-regular fa-window-minimize"></i>
            </Button>
            <Button onClick={() => appWindow.toggleMaximize() }>
                {
                    maximized ? <i className="fa-regular fa-window-restore"></i>
                    : <i className="fa-regular fa-window-maximize"></i>
                }
            </Button>
            <Button onClick={() => appWindow.close()}>
                <i className="fa-solid fa-xmark"></i>
            </Button>
        </div>
    )
}